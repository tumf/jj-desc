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
