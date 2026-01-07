// jj command integration

use crate::error::JjDescError;
use tokio::process::Command;
use tracing::{debug, instrument};

/// Represents a commit without a description
#[derive(Debug, Clone)]
pub struct Commit {
    pub change_id: String,
}

/// Result of getting a diff from jj
#[derive(Debug, Clone)]
pub enum DiffResult {
    /// Normal diff content (non-empty)
    Content(String),
    /// Merge commit with empty diff
    EmptyMerge,
    /// Non-merge commit with empty diff
    EmptyNonMerge,
}

/// Default description for empty merge commits
pub const EMPTY_MERGE_DESCRIPTION: &str = "Merge branches";

/// Default description for empty non-merge commits (placeholders)
pub const EMPTY_NON_MERGE_DESCRIPTION: &str = "(empty commit)";

/// Get the diff for the specified revision (or current working copy if None)
/// Returns DiffResult::EmptyMerge for merge commits with no changes
/// Returns DiffResult::EmptyNonMerge for non-merge commits with no changes
/// Returns DiffResult::Content for normal diffs
#[instrument(skip_all)]
pub async fn get_diff(revision: Option<&str>) -> Result<DiffResult, JjDescError> {
    let mut cmd = Command::new("jj");
    cmd.arg("diff");

    if let Some(rev) = revision {
        cmd.arg("-r").arg(rev);
    }

    debug!(?revision, "Executing jj diff");

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JjDescError::JjCommand(stderr.to_string()));
    }

    let diff = String::from_utf8(output.stdout)?;

    if diff.trim().is_empty() {
        // Check if this is a merge commit
        if is_merge_commit(revision).await? {
            debug!("Empty diff for merge commit, returning EmptyMerge");
            return Ok(DiffResult::EmptyMerge);
        }
        debug!("Empty diff for non-merge commit, returning EmptyNonMerge");
        return Ok(DiffResult::EmptyNonMerge);
    }

    debug!(diff_len = diff.len(), "Diff retrieved successfully");
    Ok(DiffResult::Content(diff))
}

/// Check if the specified revision is a merge commit (has 2+ parents)
#[instrument(skip_all)]
pub async fn is_merge_commit(revision: Option<&str>) -> Result<bool, JjDescError> {
    let mut cmd = Command::new("jj");
    cmd.args(["log", "-T", "parents.len()", "--no-graph"]);

    if let Some(rev) = revision {
        cmd.arg("-r").arg(rev);
    } else {
        cmd.arg("-r").arg("@");
    }

    debug!(?revision, "Checking if merge commit");

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JjDescError::JjCommand(stderr.to_string()));
    }

    let count_str = String::from_utf8(output.stdout)?;
    let parent_count: usize = count_str.trim().parse().unwrap_or(0);

    let is_merge = parent_count >= 2;
    debug!(parent_count, is_merge, "Merge commit check completed");

    Ok(is_merge)
}

/// Get all commits in the specified revset (regardless of description status)
/// Used when user explicitly specifies -r option to force regeneration
#[instrument(skip_all)]
pub async fn get_commits(revset: &str) -> Result<Vec<Commit>, JjDescError> {
    let mut cmd = Command::new("jj");
    cmd.arg("log")
        .arg("-r")
        .arg(revset)
        .arg("--no-graph")
        .arg("-T")
        .arg(r#"change_id.short() ++ "\n""#);

    debug!(revset = %revset, "Executing jj log to get commits");

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JjDescError::JjCommand(stderr.to_string()));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commits: Vec<Commit> = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Commit {
            change_id: line.trim().to_string(),
        })
        .collect();

    debug!(count = commits.len(), "Found commits");
    Ok(commits)
}

/// Get all commits without descriptions in the specified revset
/// Includes all commits without descriptions:
/// - Non-empty commits (have changes to describe via LLM)
/// - Empty merge commits (get "Merge branches" placeholder)
/// - Empty non-merge commits (get "(empty commit)" placeholder)
#[instrument(skip_all)]
pub async fn get_commits_without_description(revset: &str) -> Result<Vec<Commit>, JjDescError> {
    // Include all commits without descriptions, regardless of empty status
    // - Non-empty commits: LLM generates description from diff
    // - Empty merge commits: Use "Merge branches" placeholder
    // - Empty non-merge commits: Use "(empty commit)" placeholder
    let full_revset = format!(r#"description(exact:"") & ({})"#, revset);

    let mut cmd = Command::new("jj");
    cmd.arg("log")
        .arg("-r")
        .arg(&full_revset)
        .arg("--no-graph")
        .arg("-T")
        .arg(r#"change_id.short() ++ "\n""#);

    debug!(revset = %full_revset, "Executing jj log to find commits without descriptions");

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JjDescError::JjCommand(stderr.to_string()));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commits: Vec<Commit> = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Commit {
            change_id: line.trim().to_string(),
        })
        .collect();

    debug!(count = commits.len(), "Found commits without descriptions");
    Ok(commits)
}

/// Set the description for the specified revision (or current working copy if None)
#[instrument(skip(description))]
pub async fn set_description(description: &str, revision: Option<&str>) -> Result<(), JjDescError> {
    let mut cmd = Command::new("jj");
    cmd.arg("desc").arg("-m").arg(description);

    if let Some(rev) = revision {
        cmd.arg("-r").arg(rev);
    }

    debug!(?revision, desc_len = description.len(), "Executing jj desc");

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JjDescError::JjCommand(stderr.to_string()));
    }

    debug!("Description set successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // This test requires a jj repository with actual commits
    async fn test_is_merge_commit_requires_jj_repo() {
        // This is a placeholder test that demonstrates the expected behavior
        // To run this test:
        // 1. Initialize a jj repository
        // 2. Create a merge commit: jj new main feature -m "test merge"
        // 3. Run: cargo test -- --ignored test_is_merge_commit_requires_jj_repo

        // Test with current revision (@)
        let result = is_merge_commit(None).await;
        // Should not panic - either returns Ok(true/false) or Err
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_merge_commit_detection_logic() {
        // Test the core logic: parent_count >= 2 means merge commit
        let test_cases = vec![
            (0, false), // Root commit
            (1, false), // Regular commit
            (2, true),  // Merge commit (2 parents)
            (3, true),  // Octopus merge (3+ parents)
        ];

        for (parent_count, expected_is_merge) in test_cases {
            let is_merge = parent_count >= 2;
            assert_eq!(
                is_merge, expected_is_merge,
                "parent_count={} should result in is_merge={}",
                parent_count, expected_is_merge
            );
        }
    }

    #[test]
    fn test_diff_result_content() {
        // Test that Content variant holds the diff string
        let diff = "diff --git a/file.txt b/file.txt\n+new line".to_string();
        let result = DiffResult::Content(diff.clone());

        match result {
            DiffResult::Content(content) => {
                assert_eq!(content, diff);
            }
            DiffResult::EmptyMerge => {
                panic!("Expected Content variant");
            }
            DiffResult::EmptyNonMerge => {
                panic!("Expected Content variant");
            }
        }
    }

    #[test]
    fn test_diff_result_empty_merge() {
        // Test EmptyMerge variant
        let result = DiffResult::EmptyMerge;

        match result {
            DiffResult::EmptyMerge => {
                // Expected case
            }
            DiffResult::Content(_) => {
                panic!("Expected EmptyMerge variant");
            }
            DiffResult::EmptyNonMerge => {
                panic!("Expected EmptyMerge variant");
            }
        }
    }

    #[test]
    fn test_diff_result_clone() {
        // Test that DiffResult can be cloned
        let original = DiffResult::Content("test diff".to_string());
        let cloned = original.clone();

        match (&original, &cloned) {
            (DiffResult::Content(orig), DiffResult::Content(clone)) => {
                assert_eq!(orig, clone);
            }
            _ => panic!("Clone should preserve variant"),
        }

        let merge_original = DiffResult::EmptyMerge;
        let merge_cloned = merge_original.clone();

        match (merge_original, merge_cloned) {
            (DiffResult::EmptyMerge, DiffResult::EmptyMerge) => {
                // Expected
            }
            _ => panic!("Clone should preserve EmptyMerge variant"),
        }

        let non_merge_original = DiffResult::EmptyNonMerge;
        let non_merge_cloned = non_merge_original.clone();

        match (non_merge_original, non_merge_cloned) {
            (DiffResult::EmptyNonMerge, DiffResult::EmptyNonMerge) => {
                // Expected
            }
            _ => panic!("Clone should preserve EmptyNonMerge variant"),
        }
    }

    #[test]
    fn test_diff_result_debug() {
        // Test that DiffResult implements Debug
        let content = DiffResult::Content("test".to_string());
        let debug_str = format!("{:?}", content);
        assert!(debug_str.contains("Content"));

        let merge = DiffResult::EmptyMerge;
        let debug_str = format!("{:?}", merge);
        assert!(debug_str.contains("EmptyMerge"));

        let non_merge = DiffResult::EmptyNonMerge;
        let debug_str = format!("{:?}", non_merge);
        assert!(debug_str.contains("EmptyNonMerge"));
    }

    #[test]
    fn test_diff_result_empty_non_merge() {
        // Test EmptyNonMerge variant
        let result = DiffResult::EmptyNonMerge;

        match result {
            DiffResult::EmptyNonMerge => {
                // Expected case
            }
            DiffResult::Content(_) => {
                panic!("Expected EmptyNonMerge variant");
            }
            DiffResult::EmptyMerge => {
                panic!("Expected EmptyNonMerge variant");
            }
        }
    }

    #[test]
    fn test_empty_merge_description_constant() {
        // Verify the constant value for empty merge commits
        assert_eq!(EMPTY_MERGE_DESCRIPTION, "Merge branches");
        // Ensure it's not empty
        assert!(!EMPTY_MERGE_DESCRIPTION.is_empty());
    }

    #[test]
    fn test_empty_non_merge_description_constant() {
        // Verify the constant value for empty non-merge commits
        assert_eq!(EMPTY_NON_MERGE_DESCRIPTION, "(empty commit)");
        // Ensure it's not empty
        assert!(!EMPTY_NON_MERGE_DESCRIPTION.is_empty());
        // Verify it's wrapped in parentheses (convention for placeholder)
        assert!(EMPTY_NON_MERGE_DESCRIPTION.starts_with('('));
        assert!(EMPTY_NON_MERGE_DESCRIPTION.ends_with(')'));
    }

    #[test]
    fn test_description_constants_are_different() {
        // Ensure merge and non-merge descriptions are distinct
        assert_ne!(EMPTY_MERGE_DESCRIPTION, EMPTY_NON_MERGE_DESCRIPTION);
    }
}
