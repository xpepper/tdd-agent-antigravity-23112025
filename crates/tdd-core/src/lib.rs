use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};


pub mod orchestrator;
pub use orchestrator::TddOrchestrator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Tester,
    Implementor,
    Refactorer,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Tester => "tester",
            Role::Implementor => "implementor",
            Role::Refactorer => "refactorer",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepContext {
    pub role: Role,
    pub step_index: u32,
    pub kata_description: String,
    pub git_last_commit_msg: String,
    pub git_last_diff: String,
    pub repo_snapshot_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub files_changed: Vec<String>,
    pub commit_message: String,
    pub notes: String,
}

#[async_trait]
pub trait Agent: Send + Sync {
    fn role(&self) -> Role;
    async fn plan(&self, ctx: &StepContext) -> Result<String>;
    async fn edit(&self, ctx: &StepContext) -> Result<StepResult>;
}

#[async_trait]
pub trait Orchestrator {
    fn current_role(&self) -> Role;
    async fn next(&mut self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct RepoState {
    pub last_commit_message: String,
    pub last_diff: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerOutcome {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
}

#[async_trait]
pub trait Runner: Send + Sync {
    async fn fmt(&self) -> Result<RunnerOutcome>;
    async fn check(&self) -> Result<RunnerOutcome>;
    async fn test(&self) -> Result<RunnerOutcome>;
}

pub trait Vcs: Send + Sync {
    fn init_if_needed(&self) -> Result<()>;
    fn read_state(&self) -> Result<RepoState>;
    fn stage_all(&self) -> Result<()>;
    fn commit(&self, message: &str) -> Result<String>;
    fn checkout_all(&self) -> Result<()>;
}
