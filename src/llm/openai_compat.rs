// OpenAI-compatible API client (for OpenRouter, OpenAI, Gemini)

use crate::config::Config;
use crate::error::JjDescError;
use crate::llm::LlmClient;
use crate::prompt::{SYSTEM_PROMPT, build_user_prompt};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, instrument};

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// OpenAI-compatible client (works with OpenRouter, OpenAI, and Gemini)
pub struct OpenAICompatClient {
    client: Client,
    config: Config,
}

impl OpenAICompatClient {
    pub fn new(config: Config) -> Result<Self, JjDescError> {
        let client = Client::builder()
            .use_rustls_tls()
            .http1_only()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION"),
            ))
            .build()?;

        Ok(Self { client, config })
    }
}

#[async_trait]
impl LlmClient for OpenAICompatClient {
    #[instrument(skip(self, diff))]
    async fn generate_description(&self, diff: &str) -> Result<String, JjDescError> {
        let url = format!("{}/chat/completions", self.config.base_url);

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: build_user_prompt(diff),
                },
            ],
            max_tokens: self.config.max_tokens.or(Some(1024)),
            temperature: self.config.temperature.or(Some(0.3)),
        };

        debug!(
            model = %self.config.model,
            url = %url,
            provider = %self.config.provider,
            "Sending request to LLM API"
        );

        let mut req = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json");

        // Add provider-specific headers
        if self.config.provider.to_string() == "openrouter" {
            req = req
                .header("HTTP-Referer", "https://github.com/tumf/jj-desc")
                .header("X-Title", "jj-desc");
        }

        let response = req.json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(JjDescError::JjCommand(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let completion: ChatCompletionResponse = response.json().await?;

        let description = completion
            .choices
            .first()
            .ok_or_else(|| JjDescError::JjCommand("No choices in API response".to_string()))?
            .message
            .content
            .trim()
            .to_string();

        debug!(
            desc_len = description.len(),
            "Description generated successfully"
        );

        Ok(description)
    }
}
