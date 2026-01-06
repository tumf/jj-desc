// Provider enumeration and related utilities

use crate::error::JjDescError;
use std::str::FromStr;

/// Supported LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    OpenRouter,
    OpenAI,
    Anthropic,
    Gemini,
}

impl Provider {
    /// Returns the default base URL for this provider
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "https://openrouter.ai/api/v1",
            Provider::OpenAI => "https://api.openai.com/v1",
            Provider::Anthropic => "https://api.anthropic.com",
            Provider::Gemini => "https://generativelanguage.googleapis.com/v1beta/openai",
        }
    }

    /// Returns the environment variable name for the base URL of this provider
    pub fn base_url_env_var(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "OPENROUTER_BASE_URL",
            Provider::OpenAI => "OPENAI_BASE_URL",
            Provider::Anthropic => "ANTHROPIC_BASE_URL",
            Provider::Gemini => "GEMINI_BASE_URL",
        }
    }

    /// Returns the environment variable name for the API key of this provider
    pub fn api_key_env_var(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "OPENROUTER_API_KEY",
            Provider::OpenAI => "OPENAI_API_KEY",
            Provider::Anthropic => "ANTHROPIC_API_KEY",
            Provider::Gemini => "GEMINI_API_KEY",
        }
    }

    /// Returns the default model for this provider
    pub fn default_model(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "anthropic/claude-sonnet-4",
            Provider::OpenAI => "gpt-4o",
            Provider::Anthropic => "claude-sonnet-4-20250514",
            Provider::Gemini => "gemini-2.0-flash",
        }
    }

    /// Returns all available providers
    #[allow(dead_code)]
    pub fn all() -> Vec<Provider> {
        vec![
            Provider::OpenRouter,
            Provider::OpenAI,
            Provider::Anthropic,
            Provider::Gemini,
        ]
    }

    /// Returns a comma-separated list of provider names
    #[allow(dead_code)]
    pub fn list_names() -> String {
        Self::all()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl FromStr for Provider {
    type Err = JjDescError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openrouter" => Ok(Provider::OpenRouter),
            "openai" => Ok(Provider::OpenAI),
            "anthropic" => Ok(Provider::Anthropic),
            "gemini" => Ok(Provider::Gemini),
            _ => Err(JjDescError::InvalidProvider(s.to_string())),
        }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::OpenRouter => write!(f, "openrouter"),
            Provider::OpenAI => write!(f, "openai"),
            Provider::Anthropic => write!(f, "anthropic"),
            Provider::Gemini => write!(f, "gemini"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_from_str() {
        assert_eq!(
            "openrouter".parse::<Provider>().unwrap(),
            Provider::OpenRouter
        );
        assert_eq!("openai".parse::<Provider>().unwrap(), Provider::OpenAI);
        assert_eq!(
            "anthropic".parse::<Provider>().unwrap(),
            Provider::Anthropic
        );
        assert_eq!("gemini".parse::<Provider>().unwrap(), Provider::Gemini);

        // Case insensitive
        assert_eq!(
            "OpenRouter".parse::<Provider>().unwrap(),
            Provider::OpenRouter
        );
        assert_eq!("OPENAI".parse::<Provider>().unwrap(), Provider::OpenAI);

        // Invalid provider
        assert!("invalid".parse::<Provider>().is_err());
    }

    #[test]
    fn test_provider_display() {
        assert_eq!(Provider::OpenRouter.to_string(), "openrouter");
        assert_eq!(Provider::OpenAI.to_string(), "openai");
        assert_eq!(Provider::Anthropic.to_string(), "anthropic");
        assert_eq!(Provider::Gemini.to_string(), "gemini");
    }

    #[test]
    fn test_default_base_url() {
        assert_eq!(
            Provider::OpenRouter.default_base_url(),
            "https://openrouter.ai/api/v1"
        );
        assert_eq!(
            Provider::OpenAI.default_base_url(),
            "https://api.openai.com/v1"
        );
        assert_eq!(
            Provider::Anthropic.default_base_url(),
            "https://api.anthropic.com"
        );
        assert_eq!(
            Provider::Gemini.default_base_url(),
            "https://generativelanguage.googleapis.com/v1beta/openai"
        );
    }

    #[test]
    fn test_env_var_names() {
        assert_eq!(Provider::OpenRouter.api_key_env_var(), "OPENROUTER_API_KEY");
        assert_eq!(Provider::OpenAI.api_key_env_var(), "OPENAI_API_KEY");
        assert_eq!(Provider::Anthropic.api_key_env_var(), "ANTHROPIC_API_KEY");
        assert_eq!(Provider::Gemini.api_key_env_var(), "GEMINI_API_KEY");

        assert_eq!(
            Provider::OpenRouter.base_url_env_var(),
            "OPENROUTER_BASE_URL"
        );
        assert_eq!(Provider::OpenAI.base_url_env_var(), "OPENAI_BASE_URL");
        assert_eq!(Provider::Anthropic.base_url_env_var(), "ANTHROPIC_BASE_URL");
        assert_eq!(Provider::Gemini.base_url_env_var(), "GEMINI_BASE_URL");
    }

    #[test]
    fn test_default_model() {
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
    fn test_list_names() {
        let names = Provider::list_names();
        assert!(names.contains("openrouter"));
        assert!(names.contains("openai"));
        assert!(names.contains("anthropic"));
        assert!(names.contains("gemini"));
    }
}
