use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tdd_core::{Agent, Role, StepContext, StepResult};
use tdd_llm::{LlmClient, Message};
use tokio::fs;

use crate::prompts::{IMPLEMENTOR_SYSTEM_PROMPT, REFACTORER_SYSTEM_PROMPT, TESTER_SYSTEM_PROMPT};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileEdit {
    path: String,
    action: String, // "upsert"
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EditPlan {
    edits: Vec<FileEdit>,
    commit_message: String,
    notes: String,
}

pub struct LlmAgent {
    role: Role,
    llm: LlmClient,
    model: String,
    temperature: f32,
    work_dir: PathBuf,
}

impl LlmAgent {
    pub fn new(
        role: Role,
        llm: LlmClient,
        model: String,
        temperature: f32,
        work_dir: PathBuf,
    ) -> Self {
        Self {
            role,
            llm,
            model,
            temperature,
            work_dir,
        }
    }

    async fn read_files(&self, paths: &[String]) -> Result<String> {
        let mut content = String::new();
        for path in paths {
            let full_path = self.work_dir.join(path);
            if full_path.exists() {
                let file_content = fs::read_to_string(&full_path)
                    .await
                    .context(format!("Failed to read file: {}", path))?;
                content.push_str(&format!("--- {} ---\n{}\n\n", path, file_content));
            }
        }
        Ok(content)
    }

    fn system_prompt(&self) -> &'static str {
        match self.role {
            Role::Tester => TESTER_SYSTEM_PROMPT,
            Role::Implementor => IMPLEMENTOR_SYSTEM_PROMPT,
            Role::Refactorer => REFACTORER_SYSTEM_PROMPT,
        }
    }
}

#[async_trait]
impl Agent for LlmAgent {
    fn role(&self) -> Role {
        self.role
    }

    async fn plan(&self, ctx: &StepContext) -> Result<String> {
        let file_contents = self.read_files(&ctx.repo_snapshot_paths).await?;

        let user_prompt = format!(
            "Step: {}\nRole: {:?}\nKata: {}\n\nLast Commit: {}\n\nLast Diff:\n{}\n\nCurrent Files:\n{}",
            ctx.step_index,
            self.role,
            ctx.kata_description,
            ctx.git_last_commit_msg,
            ctx.git_last_diff,
            file_contents
        );

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: self.system_prompt().to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        let response = self
            .llm
            .chat(&self.model, messages, self.temperature)
            .await?;

        // Clean up response (strip markdown code blocks if present)
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        Ok(cleaned.to_string())
    }

    async fn edit(&self, ctx: &StepContext) -> Result<StepResult> {
        // Read the plan from .tdd/plan/step-N-role.md
        // Note: The orchestrator is responsible for saving the plan.
        // We assume the orchestrator saves it to a predictable path relative to work_dir.
        // Or maybe we should just pass the plan content?
        // But we are bound by the trait signature.
        // Let's assume the file exists at .tdd/plan/step-{}-{}.json (since we return JSON)
        // Wait, the requirements say "persist the plan to .tdd/plan/step-N-role.md".
        // So we read that file.

        let plan_path = self.work_dir.join(".tdd").join("plan").join(format!(
            "step-{}-{}.md",
            ctx.step_index,
            self.role.as_str()
        ));

        let plan_json = fs::read_to_string(&plan_path)
            .await
            .context(format!("Failed to read plan file: {:?}", plan_path))?;

        let plan: EditPlan =
            serde_json::from_str(&plan_json).context("Failed to parse plan JSON")?;

        let mut files_changed = Vec::new();

        for edit in plan.edits {
            if edit.action == "upsert" {
                let path = self.work_dir.join(&edit.path);
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).await?;
                }
                fs::write(&path, &edit.content).await?;
                files_changed.push(edit.path);
            }
        }

        Ok(StepResult {
            files_changed,
            commit_message: plan.commit_message,
            notes: plan.notes,
        })
    }
}
