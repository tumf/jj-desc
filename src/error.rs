// Error type definitions

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JjDescError {
    #[error(
        "API key environment variable is not set. Please set the appropriate key: OPENROUTER_API_KEY, OPENAI_API_KEY, ANTHROPIC_API_KEY, or GEMINI_API_KEY"
    )]
    MissingApiKey,

    #[error("Invalid provider: {0}. Valid providers are: openrouter, openai, anthropic, gemini")]
    InvalidProvider(String),

    #[error("Jj command failed: {0}")]
    JjCommand(String),

    #[error("API request failed with status {status}: {body}")]
    ApiStatus { status: u16, body: String },

    #[error("API response error: {0}")]
    ApiResponseError(String),

    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("Invalid UTF-8 in command output")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = JjDescError::MissingApiKey;
        assert!(
            err.to_string()
                .contains("API key environment variable is not set")
        );

        let err = JjDescError::JjCommand("test error".to_string());
        assert_eq!(err.to_string(), "Jj command failed: test error");

        let err = JjDescError::ApiStatus {
            status: 401,
            body: "Unauthorized".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "API request failed with status 401: Unauthorized"
        );

        let err = JjDescError::ApiResponseError("No choices in response".to_string());
        assert_eq!(
            err.to_string(),
            "API response error: No choices in response"
        );
    }
}
