// Anthropic Messages API client

use crate::config::Config;
use crate::error::JjDescError;
use crate::llm::LlmClient;
use crate::prompt::{SYSTEM_PROMPT, build_user_prompt};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, instrument};

const ANTHROPIC_VERSION: &str = "2023-06-01";

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[allow(dead_code)]
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

/// Anthropic Messages API client
pub struct AnthropicClient {
    client: Client,
    config: Config,
}

impl AnthropicClient {
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
impl LlmClient for AnthropicClient {
    #[instrument(skip(self, diff))]
    async fn generate_description(&self, diff: &str) -> Result<String, JjDescError> {
        let url = format!("{}/v1/messages", self.config.base_url);

        let request = AnthropicRequest {
            model: self.config.model.clone(),
            max_tokens: 1024,
            system: SYSTEM_PROMPT.to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: build_user_prompt(diff),
            }],
        };

        debug!(
            model = %self.config.model,
            url = %url,
            "Sending request to Anthropic API"
        );

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(JjDescError::ApiStatus { status, body });
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        let description = anthropic_response
            .content
            .first()
            .ok_or_else(|| {
                JjDescError::ApiResponseError("No content blocks in API response".to_string())
            })?
            .text
            .trim()
            .to_string();

        debug!(
            desc_len = description.len(),
            "Description generated successfully"
        );

        Ok(description)
    }
}
