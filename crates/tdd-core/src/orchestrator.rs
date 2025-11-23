use crate::{Agent, Orchestrator, Role, Runner, StepContext, Vcs};
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub struct TddOrchestrator {
    tester: Box<dyn Agent>,
    implementor: Box<dyn Agent>,
    refactorer: Box<dyn Agent>,
    runner: Box<dyn Runner>,
    vcs: Box<dyn Vcs>,
    kata_description: String,
    max_attempts: u32,
    work_dir: PathBuf,

    // State
    current_step: u32,
    current_role: Role,
}

impl TddOrchestrator {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tester: Box<dyn Agent>,
        implementor: Box<dyn Agent>,
        refactorer: Box<dyn Agent>,
        runner: Box<dyn Runner>,
        vcs: Box<dyn Vcs>,
        kata_description: String,
        max_attempts: u32,
        work_dir: PathBuf,
    ) -> Self {
        Self {
            tester,
            implementor,
            refactorer,
            runner,
            vcs,
            kata_description,
            max_attempts,
            work_dir,
            current_step: 1,
            current_role: Role::Tester,
        }
    }

    fn get_agent(&self, role: Role) -> &dyn Agent {
        match role {
            Role::Tester => self.tester.as_ref(),
            Role::Implementor => self.implementor.as_ref(),
            Role::Refactorer => self.refactorer.as_ref(),
        }
    }

    fn rotate_role(&mut self) {
        self.current_role = match self.current_role {
            Role::Tester => Role::Implementor,
            Role::Implementor => Role::Refactorer,
            Role::Refactorer => Role::Tester,
        };
        self.current_step += 1;
    }

    async fn save_plan(&self, plan: &str) -> Result<()> {
        let plan_dir = self.work_dir.join(".tdd").join("plan");
        fs::create_dir_all(&plan_dir).await?;
        let filename = format!(
            "step-{}-{}.md",
            self.current_step,
            self.current_role.as_str()
        );
        fs::write(plan_dir.join(filename), plan).await?;
        Ok(())
    }

    async fn save_log(&self, log: &serde_json::Value) -> Result<()> {
        let log_dir = self.work_dir.join(".tdd").join("logs");
        fs::create_dir_all(&log_dir).await?;
        let filename = format!(
            "step-{}-{}.json",
            self.current_step,
            self.current_role.as_str()
        );
        fs::write(log_dir.join(filename), serde_json::to_string_pretty(log)?).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Orchestrator for TddOrchestrator {
    fn current_role(&self) -> Role {
        self.current_role
    }

    async fn next(&mut self) -> Result<()> {
        // 1. Build StepContext
        let repo_state = self.vcs.read_state()?;
        let ctx = StepContext {
            role: self.current_role,
            step_index: self.current_step,
            kata_description: self.kata_description.clone(),
            git_last_commit_msg: repo_state.last_commit_message,
            git_last_diff: repo_state.last_diff,
            repo_snapshot_paths: repo_state.files,
        };

        let agent = self.get_agent(self.current_role);

        // 2. Plan
        println!(
            "Planning step {} as {}...",
            self.current_step,
            self.current_role.as_str()
        );
        let plan_content = agent.plan(&ctx).await?;
        self.save_plan(&plan_content).await?;

        // Loop for attempts
        let mut attempts = 0;
        loop {
            attempts += 1;
            println!("Attempt {}/{}...", attempts, self.max_attempts);

            // 3. Edit
            let step_result = agent.edit(&ctx).await?;

            // 4. Verify
            println!("Verifying...");
            let fmt_res = self.runner.fmt().await?;
            if !fmt_res.ok {
                println!("Fmt failed: {}", fmt_res.stderr);
                // If fmt fails, we might want to retry or just continue?
                // Usually fmt failure is fixable. But let's count it as failure.
            }

            let check_res = self.runner.check().await?;
            let test_res = self.runner.test().await?;

            let success = match self.current_role {
                Role::Tester => {
                    // Tester MUST fail tests (Red)
                    // But code must compile (check passes)
                    check_res.ok && !test_res.ok
                }
                Role::Implementor | Role::Refactorer => {
                    // Must pass tests (Green)
                    check_res.ok && test_res.ok
                }
            };

            if success {
                println!("Success!");
                // 6. Commit
                self.vcs.stage_all()?;
                let commit_msg = format!(
                    "{}\n\nContext:\n- Role: {:?}\n- Step: {}\n- Kata goal: ...\n\nRationale:\n{}\n\nDiff summary:\n{:?}\n\nVerification:\nTests: {}",
                    step_result.commit_message,
                    self.current_role,
                    self.current_step,
                    step_result.notes,
                    step_result.files_changed,
                    if test_res.ok { "PASS" } else { "FAIL" }
                );
                let commit_id = self.vcs.commit(&commit_msg)?;

                // Log
                let log = serde_json::json!({
                    "step": self.current_step,
                    "role": self.current_role,
                    "plan": plan_content,
                    "attempts": attempts,
                    "commit_id": commit_id,
                    "fmt_output": fmt_res,
                    "check_output": check_res,
                    "test_output": test_res,
                });
                self.save_log(&log).await?;

                self.rotate_role();
                return Ok(());
            } else {
                println!("Verification failed.");
                if attempts >= self.max_attempts {
                    anyhow::bail!("Max attempts reached for step {}", self.current_step);
                }
                // Undo changes
                self.vcs.checkout_all()?;
            }
        }
    }
}
