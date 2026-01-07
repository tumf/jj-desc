// LLM prompt generation

pub const SYSTEM_PROMPT: &str = "\
You are a helpful assistant that generates concise git commit descriptions following Conventional Commits format.
Analyze the provided diff and generate a clear, meaningful commit message.";

pub fn build_user_prompt(diff: &str) -> String {
    format!(
        r#"Generate a commit message for the following diff:

<diff>
{diff}
</diff>

Requirements:
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
- Use imperative mood in description (e.g., "add", "fix", "update")
- First line should not exceed 72 characters
- Be concise but descriptive
- Focus on the "why" and "what", not the "how"
- Include scope in parentheses if applicable (e.g., "feat(auth): add login endpoint")
- Output ONLY the commit message, without any explanation or code blocks"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user_prompt_contains_diff() {
        let diff = "diff --git a/file.rs b/file.rs\n+added line";
        let prompt = build_user_prompt(diff);

        assert!(prompt.contains(diff));
        assert!(prompt.contains("<diff>"));
        assert!(prompt.contains("</diff>"));
        assert!(prompt.contains("Conventional Commits"));
        assert!(prompt.contains("imperative mood"));
    }

    #[test]
    fn test_system_prompt_not_empty() {
        assert!(!SYSTEM_PROMPT.is_empty());
        assert!(SYSTEM_PROMPT.contains("commit"));
        assert!(SYSTEM_PROMPT.contains("Conventional Commits"));
    }
}
