// Configuration and environment variable management

use crate::error::JjDescError;
use crate::provider::Provider;
use std::env;

/// Source of configuration value (for tracking priority)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    Environment,
    CommandLine,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub provider: Provider,
    pub api_key: String,
    pub model: String,
    pub model_source: ConfigSource,
    pub base_url: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
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
        let (model, model_source) = if let Ok(m) = env::var("LLM_MODEL") {
            (m, ConfigSource::Environment)
        } else if provider == Provider::OpenRouter {
            // Backward compatibility: check OPENROUTER_MODEL
            if let Ok(m) = env::var("OPENROUTER_MODEL") {
                (m, ConfigSource::Environment)
            } else {
                (provider.default_model().to_string(), ConfigSource::Default)
            }
        } else {
            (provider.default_model().to_string(), ConfigSource::Default)
        };

        // Get base URL (priority: provider-specific env var > default)
        let base_url = env::var(provider.base_url_env_var())
            .unwrap_or_else(|_| provider.default_base_url().to_string());

        // Get LLM parameters from environment (optional)
        let max_tokens = env::var("LLM_MAX_TOKENS").ok().and_then(|s| s.parse().ok());

        let temperature = env::var("LLM_TEMPERATURE")
            .ok()
            .and_then(|s| s.parse().ok());

        Ok(Config {
            provider,
            api_key,
            model,
            model_source,
            base_url,
            max_tokens,
            temperature,
        })
    }

    /// Override the model from CLI argument
    pub fn with_model(mut self, model: Option<String>) -> Self {
        if let Some(m) = model {
            self.model = m;
            self.model_source = ConfigSource::CommandLine;
        }
        self
    }

    /// Override the provider from CLI argument
    pub fn with_provider(mut self, provider: Option<Provider>) -> Result<Self, JjDescError> {
        if let Some(p) = provider {
            self.provider = p;

            // API key must be set for the new provider
            self.api_key = env::var(p.api_key_env_var()).map_err(|_| JjDescError::MissingApiKey)?;

            // Only replace model if it was a default value (not explicitly set by user)
            if self.model_source == ConfigSource::Default {
                self.model = p.default_model().to_string();
            }
            // Otherwise keep the user's chosen model (Environment or CommandLine)

            self.base_url =
                env::var(p.base_url_env_var()).unwrap_or_else(|_| p.default_base_url().to_string());
        }
        Ok(self)
    }

    /// Override max_tokens from CLI argument
    pub fn with_max_tokens(mut self, max_tokens: Option<u32>) -> Self {
        if max_tokens.is_some() {
            self.max_tokens = max_tokens;
        }
        self
    }

    /// Override temperature from CLI argument
    pub fn with_temperature(mut self, temperature: Option<f32>) -> Self {
        if temperature.is_some() {
            self.temperature = temperature;
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
            model_source: ConfigSource::Default,
            base_url: "https://example.com".to_string(),
            max_tokens: None,
            temperature: None,
        };

        let updated = config.with_model(Some("new-model".to_string()));
        assert_eq!(updated.model, "new-model");
        assert_eq!(updated.model_source, ConfigSource::CommandLine);

        let unchanged = updated.clone().with_model(None);
        assert_eq!(unchanged.model, "new-model");
        assert_eq!(unchanged.model_source, ConfigSource::CommandLine);
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

    #[test]
    fn test_model_source_tracking() {
        // Test default model source
        unsafe {
            let original_provider = env::var("LLM_PROVIDER").ok();
            let original_api_key = env::var("OPENROUTER_API_KEY").ok();
            let original_model = env::var("LLM_MODEL").ok();

            env::set_var("OPENROUTER_API_KEY", "test-key");
            env::remove_var("LLM_MODEL");
            env::remove_var("LLM_PROVIDER");

            let config = Config::from_env().unwrap();
            assert_eq!(config.model_source, ConfigSource::Default);

            // Test environment source
            env::set_var("LLM_MODEL", "custom-model");
            let config = Config::from_env().unwrap();
            assert_eq!(config.model_source, ConfigSource::Environment);
            assert_eq!(config.model, "custom-model");

            // Restore
            env::remove_var("OPENROUTER_API_KEY");
            env::remove_var("LLM_MODEL");
            env::remove_var("LLM_PROVIDER");
            if let Some(val) = original_provider {
                env::set_var("LLM_PROVIDER", val);
            }
            if let Some(val) = original_api_key {
                env::set_var("OPENROUTER_API_KEY", val);
            }
            if let Some(val) = original_model {
                env::set_var("LLM_MODEL", val);
            }
        }
    }

    #[test]
    fn test_with_provider_preserves_user_model() {
        unsafe {
            let original_openai_key = env::var("OPENAI_API_KEY").ok();

            env::set_var("OPENAI_API_KEY", "test-key");

            // Create config with user-specified model
            let config = Config {
                provider: Provider::OpenRouter,
                api_key: "test-key".to_string(),
                model: "user-chosen-model".to_string(),
                model_source: ConfigSource::Environment,
                base_url: "https://example.com".to_string(),
                max_tokens: None,
                temperature: None,
            };

            // Switch provider - model should NOT change
            let updated = config.with_provider(Some(Provider::OpenAI)).unwrap();
            assert_eq!(updated.model, "user-chosen-model");
            assert_eq!(updated.provider, Provider::OpenAI);

            // Restore
            env::remove_var("OPENAI_API_KEY");
            if let Some(val) = original_openai_key {
                env::set_var("OPENAI_API_KEY", val);
            }
        }
    }

    #[test]
    fn test_with_provider_updates_default_model() {
        unsafe {
            let original_openai_key = env::var("OPENAI_API_KEY").ok();

            env::set_var("OPENAI_API_KEY", "test-key");

            // Create config with default model
            let config = Config {
                provider: Provider::OpenRouter,
                api_key: "test-key".to_string(),
                model: Provider::OpenRouter.default_model().to_string(),
                model_source: ConfigSource::Default,
                base_url: "https://example.com".to_string(),
                max_tokens: None,
                temperature: None,
            };

            // Switch provider - model SHOULD update to new provider's default
            let updated = config.with_provider(Some(Provider::OpenAI)).unwrap();
            assert_eq!(updated.model, Provider::OpenAI.default_model());
            assert_eq!(updated.provider, Provider::OpenAI);

            // Restore
            env::remove_var("OPENAI_API_KEY");
            if let Some(val) = original_openai_key {
                env::set_var("OPENAI_API_KEY", val);
            }
        }
    }

    #[test]
    fn test_llm_parameters_from_env() {
        unsafe {
            let original_provider = env::var("LLM_PROVIDER").ok();
            let original_api_key = env::var("OPENROUTER_API_KEY").ok();
            let original_max_tokens = env::var("LLM_MAX_TOKENS").ok();
            let original_temperature = env::var("LLM_TEMPERATURE").ok();

            env::set_var("OPENROUTER_API_KEY", "test-key");
            env::set_var("LLM_MAX_TOKENS", "2048");
            env::set_var("LLM_TEMPERATURE", "0.7");

            let config = Config::from_env().unwrap();
            assert_eq!(config.max_tokens, Some(2048));
            assert_eq!(config.temperature, Some(0.7));

            // Restore
            env::remove_var("OPENROUTER_API_KEY");
            env::remove_var("LLM_MAX_TOKENS");
            env::remove_var("LLM_TEMPERATURE");
            env::remove_var("LLM_PROVIDER");
            if let Some(val) = original_provider {
                env::set_var("LLM_PROVIDER", val);
            }
            if let Some(val) = original_api_key {
                env::set_var("OPENROUTER_API_KEY", val);
            }
            if let Some(val) = original_max_tokens {
                env::set_var("LLM_MAX_TOKENS", val);
            }
            if let Some(val) = original_temperature {
                env::set_var("LLM_TEMPERATURE", val);
            }
        }
    }

    #[test]
    fn test_with_max_tokens_and_temperature() {
        let config = Config {
            provider: Provider::OpenRouter,
            api_key: "test-key".to_string(),
            model: "default-model".to_string(),
            model_source: ConfigSource::Default,
            base_url: "https://example.com".to_string(),
            max_tokens: None,
            temperature: None,
        };

        let updated = config
            .with_max_tokens(Some(512))
            .with_temperature(Some(0.5));

        assert_eq!(updated.max_tokens, Some(512));
        assert_eq!(updated.temperature, Some(0.5));
    }
}
