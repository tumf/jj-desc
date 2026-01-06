// Error type definitions

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JjDescError {
    #[error("OPENROUTER_API_KEY environment variable is not set")]
    MissingApiKey,

    #[error("No changes found in diff")]
    EmptyDiff,

    #[error("jj command failed: {0}")]
    JjCommand(String),

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
        assert_eq!(
            err.to_string(),
            "OPENROUTER_API_KEY environment variable is not set"
        );

        let err = JjDescError::EmptyDiff;
        assert_eq!(err.to_string(), "No changes found in diff");

        let err = JjDescError::JjCommand("test error".to_string());
        assert_eq!(err.to_string(), "jj command failed: test error");
    }
}
