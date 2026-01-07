mod cli;
mod commands;
mod config;
mod error;
mod jj;
mod llm;
mod prompt;
mod provider;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Args;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    // Execute command
    commands::execute(args).await
}
