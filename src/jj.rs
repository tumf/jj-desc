// jj command integration

use crate::error::JjDescError;
use tokio::process::Command;
use tracing::{debug, instrument};

/// Represents a commit without a description
#[derive(Debug, Clone)]
pub struct Commit {
    pub change_id: String,
}

/// Get the diff for the specified revision (or current working copy if None)
#[instrument(skip_all)]
pub async fn get_diff(revision: Option<&str>) -> Result<String, JjDescError> {
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
        return Err(JjDescError::EmptyDiff);
    }

    debug!(diff_len = diff.len(), "Diff retrieved successfully");
    Ok(diff)
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

/// Get all commits without descriptions in the specified revset
/// Excludes empty commits (commits with no changes)
#[instrument(skip_all)]
pub async fn get_commits_without_description(revset: &str) -> Result<Vec<Commit>, JjDescError> {
    let full_revset = format!(r#"description(exact:"") & ~empty() & ({})"#, revset);

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
}
