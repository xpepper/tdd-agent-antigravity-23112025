use anyhow::Result;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tdd_core::{
    Agent, Orchestrator, RepoState, Role, Runner, RunnerOutcome, StepContext, StepResult,
    TddOrchestrator, Vcs,
};
use tempfile::TempDir;

struct MockAgent {
    role: Role,
}

#[async_trait]
impl Agent for MockAgent {
    fn role(&self) -> Role {
        self.role
    }

    async fn plan(&self, _ctx: &StepContext) -> Result<String> {
        Ok(format!("Plan for {:?}", self.role))
    }

    async fn edit(&self, _ctx: &StepContext) -> Result<StepResult> {
        Ok(StepResult {
            files_changed: vec!["test.rs".to_string()],
            commit_message: format!("test: add test for {:?}", self.role),
            notes: "notes".to_string(),
        })
    }
}

struct MockRunner {
    check_ok: bool,
    test_ok: bool,
}

#[async_trait]
impl Runner for MockRunner {
    async fn fmt(&self) -> Result<RunnerOutcome> {
        Ok(RunnerOutcome {
            ok: true,
            stdout: "".to_string(),
            stderr: "".to_string(),
        })
    }

    async fn check(&self) -> Result<RunnerOutcome> {
        Ok(RunnerOutcome {
            ok: self.check_ok,
            stdout: "".to_string(),
            stderr: "".to_string(),
        })
    }

    async fn test(&self) -> Result<RunnerOutcome> {
        Ok(RunnerOutcome {
            ok: self.test_ok,
            stdout: "".to_string(),
            stderr: "".to_string(),
        })
    }
}

struct MockVcs {
    commits: Arc<Mutex<Vec<String>>>,
}

impl Vcs for MockVcs {
    fn init_if_needed(&self) -> Result<()> {
        Ok(())
    }

    fn read_state(&self) -> Result<RepoState> {
        Ok(RepoState {
            last_commit_message: "initial".to_string(),
            last_diff: "".to_string(),
            files: vec![],
        })
    }

    fn stage_all(&self) -> Result<()> {
        Ok(())
    }

    fn commit(&self, message: &str) -> Result<String> {
        self.commits.lock().unwrap().push(message.to_string());
        Ok("commit-hash".to_string())
    }

    fn checkout_all(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_orchestrator_tester_step() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let work_dir = temp_dir.path().to_path_buf();

    let tester = Box::new(MockAgent { role: Role::Tester });
    let implementor = Box::new(MockAgent {
        role: Role::Implementor,
    });
    let refactorer = Box::new(MockAgent {
        role: Role::Refactorer,
    });

    // Tester expects check OK, test FAIL
    let runner = Box::new(MockRunner {
        check_ok: true,
        test_ok: false,
    });

    let commits = Arc::new(Mutex::new(Vec::new()));
    let vcs = Box::new(MockVcs {
        commits: commits.clone(),
    });

    let mut orchestrator = TddOrchestrator::new(
        tester,
        implementor,
        refactorer,
        runner,
        vcs,
        "kata".to_string(),
        3,
        work_dir,
    );

    assert_eq!(orchestrator.current_role(), Role::Tester);

    orchestrator.next().await?;

    assert_eq!(orchestrator.current_role(), Role::Implementor);
    assert_eq!(commits.lock().unwrap().len(), 1);
    assert!(commits.lock().unwrap()[0].contains("test: add test"));

    Ok(())
}
