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
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
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

    /// Files to exclude from diff (can be specified multiple times)
    #[arg(short = 'x', long = "exclude")]
    pub exclude: Vec<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
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

    /// Files to exclude from diff (can be specified multiple times)
    #[arg(short = 'x', long = "exclude")]
    pub exclude: Vec<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_backfill() {
        let args = Args::try_parse_from(&["jj-desc", "backfill"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.revisions, "::@ & mutable()");
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_revisions_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--revisions", "@"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.revisions, "@");
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_revisions_short_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "-r", "mutable()"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.revisions, "mutable()");
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_limit_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--limit", "10"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.limit, Some(10));
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_limit_short_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "-n", "5"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.limit, Some(5));
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_interactive_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--interactive"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert!(backfill_args.interactive);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_interactive_short_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "-i"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert!(backfill_args.interactive);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_dry_run_option() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--dry-run"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert!(backfill_args.dry_run);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_provider_option() {
        let args =
            Args::try_parse_from(&["jj-desc", "backfill", "--provider", "anthropic"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.provider, Some(Provider::Anthropic));
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_model_option() {
        let args = Args::try_parse_from(&[
            "jj-desc",
            "backfill",
            "--model",
            "claude-3-5-sonnet-20241022",
        ])
        .unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(
                backfill_args.model,
                Some("claude-3-5-sonnet-20241022".to_string())
            );
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_verbose_flag() {
        let args = Args::try_parse_from(&["jj-desc", "backfill", "--verbose"]).unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert!(backfill_args.verbose);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_backfill_combined_options() {
        let args = Args::try_parse_from(&[
            "jj-desc",
            "backfill",
            "-r",
            "@",
            "-n",
            "1",
            "--dry-run",
            "--interactive",
        ])
        .unwrap();
        if let Some(Command::Backfill(backfill_args)) = args.command {
            assert_eq!(backfill_args.revisions, "@");
            assert_eq!(backfill_args.limit, Some(1));
            assert!(backfill_args.dry_run);
            assert!(backfill_args.interactive);
        } else {
            panic!("Expected Backfill command");
        }
    }

    #[test]
    fn test_invalid_provider() {
        let result = Args::try_parse_from(&["jj-desc", "backfill", "--provider", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_command() {
        let args = Args::try_parse_from(&["jj-desc", "generate", "-r", "@"]).unwrap();
        if let Some(Command::Generate(generate_args)) = args.command {
            assert_eq!(generate_args.revision, Some("@".to_string()));
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_generate_exclude_option() {
        let args =
            Args::try_parse_from(&["jj-desc", "generate", "-x", "*.lock", "-x", "*.json"]).unwrap();
        if let Some(Command::Generate(generate_args)) = args.command {
            assert_eq!(generate_args.exclude, vec!["*.lock", "*.json"]);
        } else {
            panic!("Expected Generate command");
        }
    }
}
