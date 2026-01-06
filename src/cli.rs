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
    /// Preview the generated description without applying it
    #[arg(long)]
    pub dry_run: bool,

    /// LLM provider to use (openrouter, openai, anthropic, gemini)
    #[arg(long, env = "LLM_PROVIDER")]
    pub provider: Option<Provider>,

    /// Override the LLM model to use
    #[arg(long, env = "LLM_MODEL")]
    pub model: Option<String>,

    /// Target revision (defaults to current working copy)
    #[arg(short, long)]
    pub revision: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}
