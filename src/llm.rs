// OpenRouter API client

use crate::config::Config;
use crate::error::JjDescError;
use crate::prompt::{build_user_prompt, SYSTEM_PROMPT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, instrument};

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
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

pub struct OpenRouterClient {
    client: Client,
    config: Config,
}

impl OpenRouterClient {
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

    #[instrument(skip(self, diff))]
    pub async fn generate_description(&self, diff: &str) -> Result<String, JjDescError> {
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
        };

        debug!(model = %self.config.model, url = %url, "Sending request to OpenRouter");

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/tumf/jj-desc")
            .header("X-Title", "jj-desc")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
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

        debug!(desc_len = description.len(), "Description generated successfully");

        Ok(description)
    }
}
