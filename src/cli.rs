// CLI argument definitions

use crate::provider::Provider;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "jj-desc",
    version,
    about = "Generate jj commit descriptions using LLM",
    long_about = None
)]
pub struct Args {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Preview the generated descriptions without applying them
    #[arg(long)]
    pub dry_run: bool,

    /// LLM provider to use (openrouter, openai, anthropic, gemini)
    #[arg(long, env = "LLM_PROVIDER")]
    pub provider: Option<Provider>,

    /// Override the LLM model to use
    #[arg(long, env = "LLM_MODEL")]
    pub model: Option<String>,

    /// Maximum tokens for LLM response
    #[arg(long, env = "LLM_MAX_TOKENS")]
    pub max_tokens: Option<u32>,

    /// Temperature for LLM response (0.0-2.0)
    #[arg(long, env = "LLM_TEMPERATURE")]
    pub temperature: Option<f32>,

    /// Revset to select target commits (defaults to ::@ & mutable())
    #[arg(short, long, default_value = "::@ & mutable()")]
    pub revisions: String,

    /// Maximum number of commits to process
    #[arg(short = 'n', long)]
    pub limit: Option<usize>,

    /// Ask for confirmation before applying each description
    #[arg(short, long)]
    pub interactive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_revisions() {
        let args = Args::try_parse_from(&["jj-desc"]).unwrap();
        assert_eq!(args.revisions, "::@ & mutable()");
    }

    #[test]
    fn test_revisions_option() {
        let args = Args::try_parse_from(&["jj-desc", "--revisions", "@"]).unwrap();
        assert_eq!(args.revisions, "@");
    }

    #[test]
    fn test_revisions_short_option() {
        let args = Args::try_parse_from(&["jj-desc", "-r", "mutable()"]).unwrap();
        assert_eq!(args.revisions, "mutable()");
    }

    #[test]
    fn test_limit_option() {
        let args = Args::try_parse_from(&["jj-desc", "--limit", "10"]).unwrap();
        assert_eq!(args.limit, Some(10));
    }

    #[test]
    fn test_limit_short_option() {
        let args = Args::try_parse_from(&["jj-desc", "-n", "5"]).unwrap();
        assert_eq!(args.limit, Some(5));
    }

    #[test]
    fn test_interactive_option() {
        let args = Args::try_parse_from(&["jj-desc", "--interactive"]).unwrap();
        assert!(args.interactive);
    }

    #[test]
    fn test_interactive_short_option() {
        let args = Args::try_parse_from(&["jj-desc", "-i"]).unwrap();
        assert!(args.interactive);
    }

    #[test]
    fn test_dry_run_option() {
        let args = Args::try_parse_from(&["jj-desc", "--dry-run"]).unwrap();
        assert!(args.dry_run);
    }

    #[test]
    fn test_provider_option() {
        let args = Args::try_parse_from(&["jj-desc", "--provider", "anthropic"]).unwrap();
        assert_eq!(args.provider, Some(Provider::Anthropic));
    }

    #[test]
    fn test_model_option() {
        let args =
            Args::try_parse_from(&["jj-desc", "--model", "claude-3-5-sonnet-20241022"]).unwrap();
        assert_eq!(args.model, Some("claude-3-5-sonnet-20241022".to_string()));
    }

    #[test]
    fn test_verbose_flag() {
        let args = Args::try_parse_from(&["jj-desc", "--verbose"]).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn test_combined_options() {
        let args = Args::try_parse_from(&[
            "jj-desc",
            "-r",
            "@",
            "-n",
            "1",
            "--dry-run",
            "--interactive",
        ])
        .unwrap();
        assert_eq!(args.revisions, "@");
        assert_eq!(args.limit, Some(1));
        assert!(args.dry_run);
        assert!(args.interactive);
    }

    #[test]
    fn test_invalid_provider() {
        let result = Args::try_parse_from(&["jj-desc", "--provider", "invalid"]);
        assert!(result.is_err());
    }
}
