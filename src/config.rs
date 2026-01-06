// Configuration and environment variable management

use crate::error::JjDescError;
use crate::provider::Provider;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub provider: Provider,
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, JjDescError> {
        // Determine provider
        let provider = env::var("LLM_PROVIDER")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Provider::OpenRouter);

        // Get API key for the selected provider
        let api_key = env::var(provider.api_key_env_var())
            // Fallback to OPENROUTER_API_KEY for backward compatibility
            .or_else(|_| {
                if provider == Provider::OpenRouter {
                    env::var("OPENROUTER_API_KEY")
                } else {
                    Err(env::VarError::NotPresent)
                }
            })
            .map_err(|_| JjDescError::MissingApiKey)?;

        // Get model (priority: LLM_MODEL > provider-specific env var > default)
        let model = env::var("LLM_MODEL")
            .ok()
            .or_else(|| {
                // Backward compatibility: check OPENROUTER_MODEL
                if provider == Provider::OpenRouter {
                    env::var("OPENROUTER_MODEL").ok()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| provider.default_model().to_string());

        // Get base URL (priority: provider-specific env var > default)
        let base_url = env::var(provider.base_url_env_var())
            .unwrap_or_else(|_| provider.default_base_url().to_string());

        Ok(Config {
            provider,
            api_key,
            model,
            base_url,
        })
    }

    /// Override the model from CLI argument
    pub fn with_model(mut self, model: Option<String>) -> Self {
        if let Some(m) = model {
            self.model = m;
        }
        self
    }

    /// Override the provider from CLI argument
    pub fn with_provider(mut self, provider: Option<Provider>) -> Self {
        if let Some(p) = provider {
            self.provider = p;
            // Update API key, model, and base_url for the new provider if needed
            if let Ok(api_key) = env::var(p.api_key_env_var()) {
                self.api_key = api_key;
            }
            if self.model == Provider::OpenRouter.default_model()
                || self.model == Provider::OpenAI.default_model()
                || self.model == Provider::Anthropic.default_model()
                || self.model == Provider::Gemini.default_model()
            {
                // If using a default model, update to the new provider's default
                self.model = p.default_model().to_string();
            }
            self.base_url =
                env::var(p.base_url_env_var()).unwrap_or_else(|_| p.default_base_url().to_string());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_with_model() {
        let config = Config {
            provider: Provider::OpenRouter,
            api_key: "test-key".to_string(),
            model: "default-model".to_string(),
            base_url: "https://example.com".to_string(),
        };

        let updated = config.with_model(Some("new-model".to_string()));
        assert_eq!(updated.model, "new-model");

        let unchanged = updated.clone().with_model(None);
        assert_eq!(unchanged.model, "new-model");
    }

    #[test]
    fn test_provider_defaults() {
        assert_eq!(
            Provider::OpenRouter.default_model(),
            "anthropic/claude-sonnet-4"
        );
        assert_eq!(Provider::OpenAI.default_model(), "gpt-4o");
        assert_eq!(
            Provider::Anthropic.default_model(),
            "claude-sonnet-4-20250514"
        );
        assert_eq!(Provider::Gemini.default_model(), "gemini-2.0-flash");
    }

    #[test]
    fn test_custom_base_url() {
        // This test modifies environment variables, which is inherently unsafe in a
        // multi-threaded test environment. Run with `--test-threads=1` if needed.
        unsafe {
            // Save original env
            let original_provider = env::var("LLM_PROVIDER").ok();
            let original_api_key = env::var("OPENAI_API_KEY").ok();
            let original_base_url = env::var("OPENAI_BASE_URL").ok();

            // Set test environment
            env::set_var("LLM_PROVIDER", "openai");
            env::set_var("OPENAI_API_KEY", "test-key");
            env::set_var("OPENAI_BASE_URL", "https://custom.openai.com");

            let config = Config::from_env().unwrap();
            assert_eq!(config.provider, Provider::OpenAI);
            assert_eq!(config.base_url, "https://custom.openai.com");

            // Restore original env
            env::remove_var("LLM_PROVIDER");
            env::remove_var("OPENAI_API_KEY");
            env::remove_var("OPENAI_BASE_URL");
            if let Some(val) = original_provider {
                env::set_var("LLM_PROVIDER", val);
            }
            if let Some(val) = original_api_key {
                env::set_var("OPENAI_API_KEY", val);
            }
            if let Some(val) = original_base_url {
                env::set_var("OPENAI_BASE_URL", val);
            }
        }
    }
}
