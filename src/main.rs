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
use cli::{Args, Command, GenerateArgs};
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

    // Execute command (default to Generate if no subcommand provided)
    match args.command {
        Some(Command::Generate(generate_args)) => commands::execute_generate(generate_args).await,
        Some(Command::Backfill(backfill_args)) => commands::execute_backfill(backfill_args).await,
        None => {
            // Backward compatibility: no subcommand means generate with defaults
            let generate_args = GenerateArgs {
                dry_run: false,
                provider: None,
                model: None,
                revision: None,
            };
            commands::execute_generate(generate_args).await
        }
    }
}
