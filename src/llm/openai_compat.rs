// OpenAI-compatible API client (for OpenRouter, OpenAI, Gemini)

use crate::config::Config;
use crate::error::JjDescError;
use crate::llm::{
    DEFAULT_CONNECT_TIMEOUT_SECS, DEFAULT_MAX_TOKENS, DEFAULT_REQUEST_TIMEOUT_SECS,
    DEFAULT_TEMPERATURE, LlmClient, build_http_client,
};
use crate::prompt::{SYSTEM_PROMPT, build_user_prompt};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
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
        let client = build_http_client(DEFAULT_REQUEST_TIMEOUT_SECS, DEFAULT_CONNECT_TIMEOUT_SECS)?;

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
            max_tokens: self.config.max_tokens.or(Some(DEFAULT_MAX_TOKENS)),
            temperature: self.config.temperature.or(Some(DEFAULT_TEMPERATURE)),
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
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(JjDescError::ApiStatus { status, body });
        }

        let completion: ChatCompletionResponse = response.json().await?;

        let description = completion
            .choices
            .first()
            .ok_or_else(|| JjDescError::ApiResponseError("No choices in API response".to_string()))?
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::test_config;
    use crate::provider::Provider;

    #[test]
    fn test_client_initialization_openai() {
        let config = test_config(Provider::OpenAI);
        let result = OpenAICompatClient::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_client_initialization_openrouter() {
        let config = test_config(Provider::OpenRouter);
        let result = OpenAICompatClient::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_client_initialization_gemini() {
        let config = test_config(Provider::Gemini);
        let result = OpenAICompatClient::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_client_with_custom_model() {
        let mut config = test_config(Provider::OpenAI);
        config.model = "gpt-4-turbo".to_string();
        let client = OpenAICompatClient::new(config.clone()).unwrap();
        assert_eq!(client.config.model, "gpt-4-turbo");
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let mut config = test_config(Provider::OpenAI);
        config.base_url = "https://custom.openai.com/v1".to_string();
        let client = OpenAICompatClient::new(config.clone()).unwrap();
        assert_eq!(client.config.base_url, "https://custom.openai.com/v1");
    }

    #[test]
    fn test_request_structure() {
        let request = ChatCompletionRequest {
            model: "gpt-4o".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: "test diff".to_string(),
                },
            ],
            max_tokens: None,
            temperature: None,
        };

        // Verify serialization works
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-4o"));
        assert!(json.contains("system"));
        assert!(json.contains("user"));
    }
}
