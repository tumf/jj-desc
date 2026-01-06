// LLM client abstraction and implementations

use crate::config::Config;
use crate::error::JjDescError;
use crate::provider::Provider;
use async_trait::async_trait;

/// Trait for LLM clients that can generate commit descriptions
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Generate a commit description from a diff
    async fn generate_description(&self, diff: &str) -> Result<String, JjDescError>;
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
mod tests {
    use super::*;

    fn test_config(provider: Provider) -> Config {
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
