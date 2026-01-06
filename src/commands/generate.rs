// Generate description for a single commit

use anyhow::{Context, Result};
use tracing::info;

use crate::cli::GenerateArgs;
use crate::config::Config;
use crate::error;
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
                return Err(error::JjDescError::EmptyDiff).context("No changes found in diff");
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
