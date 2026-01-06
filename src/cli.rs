// CLI argument definitions

use crate::provider::Provider;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "jj-desc",
    version,
    about = "Generate jj commit descriptions using LLM",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate description for a single commit
    Generate(GenerateArgs),

    /// Backfill descriptions for multiple commits (default)
    Backfill(BackfillArgs),
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Preview the generated description without applying it
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

    /// Target revision (defaults to current working copy)
    #[arg(short, long)]
    pub revision: Option<String>,
}

#[derive(Parser, Debug)]
pub struct BackfillArgs {
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
    fn test_generate_subcommand() {
        let args = Args::try_parse_from(&["jj-desc", "generate"]).unwrap();
        assert!(matches!(args.command, Some(Command::Generate(_))));
    }

    #[test]
    fn test_generate_with_revision() {
        let args = Args::try_parse_from(&["jj-desc", "generate", "--revision", "@"]).unwrap();
        if let Some(Command::Generate(generate_args)) = args.command {
            assert_eq!(generate_args.revision, Some("@".to_string()));
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_generate_with_provider() {
        let args =
            Args::try_parse_from(&["jj-desc", "generate", "--provider", "anthropic"]).unwrap();
        if let Some(Command::Generate(generate_args)) = args.command {
            assert_eq!(generate_args.provider, Some(Provider::Anthropic));
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_generate_with_model() {
        let args = Args::try_parse_from(&[
            "jj-desc",
            "generate",
            "--model",
            "claude-3-5-sonnet-20241022",
        ])
        .unwrap();
        if let Some(Command::Generate(generate_args)) = args.command {
            assert_eq!(
                generate_args.model,
                Some("claude-3-5-sonnet-20241022".to_string())
            );
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_generate_with_dry_run() {
        let args = Args::try_parse_from(&["jj-desc", "generate", "--dry-run"]).unwrap();
        if let Some(Command::Generate(generate_args)) = args.command {
            assert!(generate_args.dry_run);
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_backfill_subcommand() {
        let args = Args::try_parse_from(&["jj-desc", "backfill"]).unwrap();
        assert!(matches!(args.command, Some(Command::Backfill(_))));
    }

    #[test]
    fn test_backfill_with_revisions() {
        let args =
            Args::try_parse_from(&["jj-desc", "backfill", "--revisions", "mutable()"]).unwrap();
        if let Some(Command::Backfill(backfill)) = args.command {
            assert_eq!(backfill.revisions, "mutable()");
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_with_dry_run() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--dry-run"]).unwrap();
        if let Some(Command::Backfill(backfill)) = args.command {
            assert!(backfill.dry_run);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_with_interactive() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--interactive"]).unwrap();
        if let Some(Command::Backfill(backfill)) = args.command {
            assert!(backfill.interactive);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_with_limit() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--limit", "10"]).unwrap();
        if let Some(Command::Backfill(backfill)) = args.command {
            assert_eq!(backfill.limit, Some(10));
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_verbose_flag() {
        let args = Args::try_parse_from(&["jj-desc", "--verbose", "generate"]).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn test_invalid_provider() {
        let result = Args::try_parse_from(&["jj-desc", "generate", "--provider", "invalid"]);
        assert!(result.is_err());
    }
}
