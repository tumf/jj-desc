// LLM client abstraction and implementations

use crate::config::Config;
use crate::error::JjDescError;
use crate::provider::Provider;
use async_trait::async_trait;

/// Default maximum tokens for LLM responses
pub const DEFAULT_MAX_TOKENS: u32 = 1024;

/// Default temperature for LLM requests
pub const DEFAULT_TEMPERATURE: f32 = 0.3;

/// Default HTTP request timeout in seconds
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// Default HTTP connection timeout in seconds
pub const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 5;

/// Trait for LLM clients that can generate commit descriptions
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Generate a commit description from a diff
    async fn generate_description(&self, diff: &str) -> Result<String, JjDescError>;
}

/// Build an HTTP client with common configuration
pub fn build_http_client(
    timeout_secs: u64,
    connect_timeout_secs: u64,
) -> Result<reqwest::Client, JjDescError> {
    reqwest::Client::builder()
        .use_rustls_tls()
        .http1_only()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .connect_timeout(std::time::Duration::from_secs(connect_timeout_secs))
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .build()
        .map_err(|e| e.into())
}

mod anthropic;
mod openai_compat;

pub use anthropic::AnthropicClient;
pub use openai_compat::OpenAICompatClient;

/// Create an LLM client based on the provider in the config
pub fn create_client(config: Config) -> Result<Box<dyn LlmClient>, JjDescError> {
    match config.provider {
        Provider::OpenRouter | Provider::OpenAI | Provider::Gemini => {
            Ok(Box::new(OpenAICompatClient::new(config)?))
        }
        Provider::Anthropic => Ok(Box::new(AnthropicClient::new(config)?)),
    }
}

#[cfg(test)]
pub(crate) fn test_config(provider: Provider) -> Config {
    Config {
        provider,
        api_key: "test-key".to_string(),
        model: provider.default_model().to_string(),
        model_source: crate::config::ConfigSource::Default,
        base_url: provider.default_base_url().to_string(),
        max_tokens: None,
        temperature: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_anthropic_client() {
        let config = test_config(Provider::Anthropic);
        let result = create_client(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_openai_client() {
        let config = test_config(Provider::OpenAI);
        let result = create_client(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_openrouter_client() {
        let config = test_config(Provider::OpenRouter);
        let result = create_client(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_gemini_client() {
        let config = test_config(Provider::Gemini);
        let result = create_client(config);
        assert!(result.is_ok());
    }
}
