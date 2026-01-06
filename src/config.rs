// Configuration and environment variable management

use crate::error::JjDescError;
use std::env;

const DEFAULT_MODEL: &str = "anthropic/claude-sonnet-4";
const DEFAULT_BASE_URL: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, JjDescError> {
        let api_key = env::var("OPENROUTER_API_KEY").map_err(|_| JjDescError::MissingApiKey)?;

        let model = env::var("OPENROUTER_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

        let base_url =
            env::var("OPENROUTER_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Ok(Config {
            api_key,
            model,
            base_url,
        })
    }

    pub fn with_model(mut self, model: Option<String>) -> Self {
        if let Some(m) = model {
            self.model = m;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_model() {
        let config = Config {
            api_key: "test-key".to_string(),
            model: "default-model".to_string(),
            base_url: "https://example.com".to_string(),
        };

        let updated = config.with_model(Some("new-model".to_string()));
        assert_eq!(updated.model, "new-model");

        let unchanged = updated.clone().with_model(None);
        assert_eq!(unchanged.model, "new-model");
    }
}
