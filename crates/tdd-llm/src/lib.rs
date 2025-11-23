use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct LlmClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl LlmClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap_or_default();
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        }
    }

    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
    ) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send LLM request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("LLM API error: {}", error_text);
        }

        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse LLM response")?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .context("No content in LLM response")?
            .to_string();

        Ok(content)
    }
}
