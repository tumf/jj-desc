// Generate description for a single commit

use anyhow::{Context, Result};
use tracing::info;

use crate::cli::GenerateArgs;
use crate::config::Config;
use crate::{jj, llm};

pub async fn execute_generate(args: GenerateArgs) -> Result<()> {
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
    let diff = jj::get_diff(revision)
        .await
        .context("Failed to get diff from jj")?;

    info!("Retrieved diff ({} bytes)", diff.len());

    // Generate description using LLM
    let client = llm::create_client(config).context("Failed to create LLM client")?;

    let description = client
        .generate_description(&diff)
        .await
        .context("Failed to generate description")?;

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
