// LLM prompt generation

use clap::ValueEnum;

/// Commit message style the LLM should follow
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum CommitStyle {
    /// Conventional Commits: `<type>[optional scope]: <description>`
    #[default]
    Conventional,
    /// Scoped Commits (https://scopedcommits.com): `<scope>: <description>`
    Scoped,
}

impl CommitStyle {
    /// Returns the system prompt for this style
    pub fn system_prompt(&self) -> &'static str {
        match self {
            CommitStyle::Conventional => CONVENTIONAL_SYSTEM_PROMPT,
            CommitStyle::Scoped => SCOPED_SYSTEM_PROMPT,
        }
    }

    /// Returns the style-specific format requirements for the user prompt
    fn format_requirements(&self) -> &'static str {
        match self {
            CommitStyle::Conventional => CONVENTIONAL_REQUIREMENTS,
            CommitStyle::Scoped => SCOPED_REQUIREMENTS,
        }
    }
}

const CONVENTIONAL_SYSTEM_PROMPT: &str = "\
You are a helpful assistant that generates concise git commit descriptions following Conventional Commits format.
Analyze the provided diff and generate a clear, meaningful commit message.";

const SCOPED_SYSTEM_PROMPT: &str = "\
You are a helpful assistant that generates concise git commit descriptions following Scoped Commits format.
Analyze the provided diff and generate a clear, meaningful commit message.";

const CONVENTIONAL_REQUIREMENTS: &str = "\
- Follow Conventional Commits format: <type>[optional scope]: <description>
- Use one of these types:
  * feat: New feature
  * fix: Bug fix
  * docs: Documentation only changes
  * style: Code style changes (formatting, whitespace, etc.)
  * refactor: Code refactoring (neither fixes a bug nor adds a feature)
  * perf: Performance improvements
  * test: Adding or modifying tests
  * chore: Build process or tooling changes
  * ci: CI configuration changes
  * build: Changes to build system or dependencies
- Include scope in parentheses if applicable (e.g., \"feat(auth): add login endpoint\")";

const SCOPED_REQUIREMENTS: &str = "\
- Follow Scoped Commits format (https://scopedcommits.com):

  <scope>: <description>

  [optional body]

  [optional trailer(s)]

- The scope is the subsystem, area, or module the change touches, derived from the
  changed paths (e.g., a crate, package, directory, or component name such as
  \"auth\", \"net/http\", or \"ci\")
- Do NOT use a Conventional Commits type prefix (no \"feat:\", \"fix:\", etc.)
- If the change spans several areas, prefer a more general common scope, or list
  the scopes separated by a comma (e.g., \"cli, config: ...\"); for tree-wide
  changes use \"treewide\"
- Start the description in lowercase with no trailing period
  (e.g., \"auth: fix token refresh on expiry\")
- Separate the optional body from the header with a blank line";

/// Builds the user prompt for the given diff and commit style
pub fn build_user_prompt(diff: &str, style: CommitStyle) -> String {
    let format_requirements = style.format_requirements();
    format!(
        r#"Generate a commit message for the following diff:

<diff>
{diff}
</diff>

Requirements:
{format_requirements}
- Use imperative mood in description (e.g., "add", "fix", "update")
- First line should not exceed 72 characters
- Be concise but descriptive
- Focus on the "why" and "what", not the "how"
- Output ONLY the commit message, without any explanation or code blocks"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user_prompt_contains_diff() {
        let diff = "diff --git a/file.rs b/file.rs\n+added line";
        let prompt = build_user_prompt(diff, CommitStyle::Conventional);

        assert!(prompt.contains(diff));
        assert!(prompt.contains("<diff>"));
        assert!(prompt.contains("</diff>"));
        assert!(prompt.contains("Conventional Commits"));
        assert!(prompt.contains("imperative mood"));
    }

    #[test]
    fn test_build_user_prompt_scoped() {
        let diff = "diff --git a/file.rs b/file.rs\n+added line";
        let prompt = build_user_prompt(diff, CommitStyle::Scoped);

        assert!(prompt.contains(diff));
        assert!(prompt.contains("Scoped Commits"));
        assert!(prompt.contains("<scope>: <description>"));
        assert!(prompt.contains("imperative mood"));
        assert!(!prompt.contains("feat: New feature"));
    }

    #[test]
    fn test_system_prompt_not_empty() {
        let conventional = CommitStyle::Conventional.system_prompt();
        assert!(conventional.contains("commit"));
        assert!(conventional.contains("Conventional Commits"));

        let scoped = CommitStyle::Scoped.system_prompt();
        assert!(scoped.contains("commit"));
        assert!(scoped.contains("Scoped Commits"));
    }

    #[test]
    fn test_default_style_is_conventional() {
        assert_eq!(CommitStyle::default(), CommitStyle::Conventional);
    }
}
