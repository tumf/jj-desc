// jj command integration

use crate::error::JjDescError;
use tokio::process::Command;
use tracing::{debug, instrument};

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

/// Set the description for the specified revision (or current working copy if None)
#[instrument(skip(description))]
pub async fn set_description(
    description: &str,
    revision: Option<&str>,
) -> Result<(), JjDescError> {
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
