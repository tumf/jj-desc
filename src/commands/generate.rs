// Generate description for a single commit

use anyhow::{Context, Result};
use tracing::info;

use crate::cli::GenerateArgs;
use crate::config::Config;
use crate::jj::{DiffResult, EMPTY_MERGE_DESCRIPTION, EMPTY_NON_MERGE_DESCRIPTION};
use crate::{jj, llm};

pub async fn execute_generate(args: GenerateArgs) -> Result<()> {
    // Load configuration
    let config = Config::from_env()
        .context("Failed to load configuration")?
        .with_provider(args.provider)
        .context("Failed to set provider")?
        .with_model(args.model)
        .with_max_tokens(args.max_tokens)
        .with_temperature(args.temperature);

    info!(
        "Using provider: {}, model: {}, base_url: {}",
        config.provider, config.model, config.base_url
    );

    // Get diff from jj
    let revision = args.revision.as_deref();

    let diff_result = jj::get_diff(revision)
        .await
        .context("Failed to get diff from jj")?;

    let description = match diff_result {
        DiffResult::Content(diff) => {
            info!("Retrieved diff ({} bytes)", diff.len());

            // Generate description using LLM
            let client = llm::create_client(config).context("Failed to create LLM client")?;

            client
                .generate_description(&diff)
                .await
                .context("Failed to generate description")?
        }
        DiffResult::EmptyMerge => {
            info!("Empty merge commit detected, using default description");
            EMPTY_MERGE_DESCRIPTION.to_string()
        }
        DiffResult::EmptyNonMerge => {
            info!("Empty non-merge commit detected, using placeholder description");
            EMPTY_NON_MERGE_DESCRIPTION.to_string()
        }
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
