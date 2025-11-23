use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfig {
    pub model: String,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub base_url: String,
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiConfig {
    pub test_cmd: Vec<String>,
    pub check_cmd: Vec<String>,
    pub fmt_cmd: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitConfig {
    pub author_name: String,
    pub author_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub kata_description: String,
    pub language: String,
    pub steps: u32,
    pub max_attempts_per_agent: u32,
    pub roles: HashMap<String, RoleConfig>,
    pub llm: LlmConfig,
    pub ci: CiConfig,
    pub commit: CommitConfig,
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)
            .await
            .context("Failed to read config file")?;
        let config: Config =
            serde_yaml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn default_yaml() -> &'static str {
        r#"kata_description: "kata.md"
language: "rust"
steps: 20
max_attempts_per_agent: 5
roles:
  tester:
    model: "openai:gpt-4o"
    temperature: 0.4
  implementor:
    model: "openai:gpt-4o"
    temperature: 0.2
  refactorer:
    model: "openai:gpt-4o"
    temperature: 0.3
llm:
  base_url: "https://api.openai.com/v1"
  api_key_env: "OPENAI_API_KEY"
ci:
  test_cmd: ["cargo", "test", "--all"]
  check_cmd: ["cargo", "clippy", "--all", "--", "-D", "warnings"]
  fmt_cmd: ["cargo", "fmt"]
commit:
  author_name: "TDD Machine"
  author_email: "tdd@local"
"#
    }
}
