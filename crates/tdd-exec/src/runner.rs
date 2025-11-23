use anyhow::{Context, Result};
use std::process::Stdio;
use tdd_core::{Runner, RunnerOutcome};
use tokio::process::Command;

pub struct ProcessRunner {
    fmt_cmd: Vec<String>,
    check_cmd: Vec<String>,
    test_cmd: Vec<String>,
}

impl ProcessRunner {
    pub fn new(fmt_cmd: Vec<String>, check_cmd: Vec<String>, test_cmd: Vec<String>) -> Self {
        Self {
            fmt_cmd,
            check_cmd,
            test_cmd,
        }
    }

    async fn run_command(&self, cmd_parts: &[String]) -> Result<RunnerOutcome> {
        if cmd_parts.is_empty() {
            return Ok(RunnerOutcome {
                ok: true,
                stdout: String::new(),
                stderr: String::new(),
            });
        }

        let program = &cmd_parts[0];
        let args = &cmd_parts[1..];

        let output = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(format!("Failed to spawn command: {}", program))?
            .wait_with_output()
            .await
            .context(format!("Failed to wait for command: {}", program))?;

        Ok(RunnerOutcome {
            ok: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Runner for ProcessRunner {
    async fn fmt(&self) -> Result<RunnerOutcome> {
        self.run_command(&self.fmt_cmd).await
    }

    async fn check(&self) -> Result<RunnerOutcome> {
        self.run_command(&self.check_cmd).await
    }

    async fn test(&self) -> Result<RunnerOutcome> {
        self.run_command(&self.test_cmd).await
    }
}
