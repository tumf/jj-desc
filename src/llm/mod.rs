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
