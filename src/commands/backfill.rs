// Backfill descriptions for multiple commits

use anyhow::{Context, Result};
use tracing::{info, warn};

use crate::cli::BackfillArgs;
use crate::config::Config;
use crate::jj::DiffResult;
use crate::{jj, llm};

pub async fn execute_backfill(args: BackfillArgs) -> Result<()> {
    // Load configuration
    let config = Config::from_env()
        .context("Failed to load configuration")?
        .with_provider(args.provider)
        .context("Failed to set provider")?
        .with_model(args.model)
        .with_max_tokens(args.max_tokens)
        .with_temperature(args.temperature);

    info!(
        "Using provider: {}, model: {}",
        config.provider, config.model
    );

    // Get commits without descriptions
    let commits = jj::get_commits_without_description(&args.revisions)
        .await
        .context("Failed to get commits without descriptions")?;

    if commits.is_empty() {
        println!(
            "No commits without descriptions found in revset: {}",
            args.revisions
        );
        return Ok(());
    }

    println!("Found {} commit(s) without descriptions", commits.len());

    // Apply limit if specified
    let commits_to_process: Vec<_> = if let Some(limit) = args.limit {
        commits.into_iter().take(limit).collect()
    } else {
        commits
    };

    if args.limit.is_some() {
        println!("Processing {} commit(s)", commits_to_process.len());
    }

    // Create LLM client
    let client = llm::create_client(config).context("Failed to create LLM client")?;

    // Track results
    let mut success_count = 0;
    let mut skip_count = 0;
    let mut failure_count = 0;
    let total = commits_to_process.len();

    // Process each commit
    for (index, commit) in commits_to_process.iter().enumerate() {
        let progress = index + 1;
        println!(
            "\nProcessing: {}/{} ({}%)",
            progress,
            total,
            progress * 100 / total
        );
        println!("Commit: {}", commit.change_id);

        // Get diff for this commit
        let diff_result = match jj::get_diff(Some(&commit.change_id)).await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("✗ Failed to get diff: {}", e);
                failure_count += 1;
                continue;
            }
        };

        // Generate description based on diff result
        let description = match diff_result {
            DiffResult::Content(diff) => match client.generate_description(&diff).await {
                Ok(desc) => desc,
                Err(e) => {
                    eprintln!("✗ Failed to generate description: {}", e);
                    failure_count += 1;
                    continue;
                }
            },
            DiffResult::EmptyMerge => {
                // For empty merge commits, use a default description
                "Merge branches".to_string()
            }
            DiffResult::EmptyNonMerge => {
                // For empty non-merge commits, skip them
                println!("○ Skipped (empty commit)");
                skip_count += 1;
                continue;
            }
        };

        // Display generated description
        println!("Generated description:");
        println!("  {}", description.lines().next().unwrap_or(""));

        // Handle interactive mode
        if args.interactive {
            println!("\nFull description:");
            println!("─────────────────────");
            println!("{}", description);
            println!("─────────────────────");
            println!("Accept (a) / Skip (s) / Quit (q): ");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .context("Failed to read input")?;

            match input.trim().to_lowercase().as_str() {
                "a" | "accept" => {
                    // Continue to apply
                }
                "s" | "skip" => {
                    println!("○ Skipped");
                    skip_count += 1;
                    continue;
                }
                "q" | "quit" => {
                    println!("Quitting...");
                    break;
                }
                _ => {
                    println!("○ Skipped (invalid input)");
                    skip_count += 1;
                    continue;
                }
            }
        }

        // Apply description if not dry-run
        if args.dry_run {
            println!("○ (Dry-run, not applied)");
            skip_count += 1;
        } else {
            match jj::set_description(&description, Some(&commit.change_id)).await {
                Ok(_) => {
                    println!("✓ Description applied");
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("✗ Failed to set description: {}", e);
                    failure_count += 1;
                }
            }
        }
    }

    // Print summary
    println!("\n═══════════════════════");
    println!("Summary:");
    println!("  Success:  {}", success_count);
    println!("  Skipped:  {}", skip_count);
    println!("  Failed:   {}", failure_count);
    println!("═══════════════════════");

    if failure_count > 0 {
        warn!("{} commit(s) failed to process", failure_count);
    }

    Ok(())
}
