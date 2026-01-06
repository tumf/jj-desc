mod cli;
mod config;
mod error;
mod jj;
mod llm;
mod prompt;
mod provider;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Args;
use config::Config;
use tracing::{Level, info};
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

    // Load configuration
    let config = Config::from_env()
        .context("Failed to load configuration")?
        .with_provider(args.provider)
        .with_model(args.model);

    info!(
        "Using provider: {}, model: {}, base_url: {}",
        config.provider, config.model, config.base_url
    );

    // Get diff from jj
    let revision = args.revision.as_deref();
    
    // Try to get the diff, but handle empty diffs specially for merge commits
    let diff_result = jj::get_diff(revision).await;
    
    let description = match diff_result {
        Ok(diff) => {
            info!("Retrieved diff ({} bytes)", diff.len());
            
            // Generate description using LLM
            let client = llm::create_client(config).context("Failed to create LLM client")?;
            
            client
                .generate_description(&diff)
                .await
                .context("Failed to generate description")?
        }
        Err(error::JjDescError::EmptyDiff) => {
            // Check if this is a merge commit
            if jj::is_merge_commit(revision).await? {
                info!("Empty diff detected, but this is a merge commit");
                "Merge commit".to_string()
            } else {
                return Err(error::JjDescError::EmptyDiff)
                    .context("No changes found in diff");
            }
        }
        Err(e) => return Err(e).context("Failed to get diff from jj"),
    };

    // Display or apply the description
    if args.dry_run {
        println!("\nGenerated description (not applied):");
        println!("─────────────────────");
        println!("{}", description);
    } else {
        jj::set_description(&description, revision)
            .await
            .context("Failed to set description")?;

        println!("\nApplied description:");
        println!("─────────────────────");
        println!("{}", description);
    }

    Ok(())
}
