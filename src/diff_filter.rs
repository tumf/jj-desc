// Diff filtering and optimization for large diffs

use tracing::{debug, warn};

/// Default file patterns to exclude from diffs (lock files)
const DEFAULT_EXCLUDES: &[&str] = &[
    "Cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "*.lock",
    "*.lockb",
];

/// Warning threshold for large diffs (50KB)
const WARNING_THRESHOLD: usize = 50 * 1024;

/// Result of diff filtering with statistics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilteredDiff {
    /// Filtered diff content
    pub content: String,
    /// Original diff size in bytes
    pub original_size: usize,
    /// Filtered diff size in bytes
    pub filtered_size: usize,
    /// List of excluded files
    pub excluded_files: Vec<String>,
    /// List of binary files that were simplified
    pub binary_files: Vec<String>,
}

impl FilteredDiff {
    /// Get the number of lines in the filtered diff
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// Check if any files were excluded
    pub fn has_exclusions(&self) -> bool {
        !self.excluded_files.is_empty() || !self.binary_files.is_empty()
    }

    /// Get reduction percentage
    /// Returns 0.0 if filtered size is greater than or equal to original size
    pub fn reduction_percentage(&self) -> f64 {
        if self.original_size == 0 || self.filtered_size >= self.original_size {
            return 0.0;
        }
        let reduced = self.original_size - self.filtered_size;
        (reduced as f64 / self.original_size as f64) * 100.0
    }
}

/// Filter diff content by excluding lock files and simplifying binary files
///
/// # Arguments
/// * `raw_diff` - Raw diff output from jj
/// * `exclude_patterns` - Additional user-specified exclusion patterns
///
/// # Returns
/// FilteredDiff containing filtered content and statistics
pub fn filter_diff(raw_diff: &str, exclude_patterns: &[String]) -> FilteredDiff {
    let original_size = raw_diff.len();

    debug!(
        original_size,
        exclude_count = exclude_patterns.len(),
        "Filtering diff"
    );

    // Combine default and user-specified exclusion patterns
    let mut all_patterns = DEFAULT_EXCLUDES
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    all_patterns.extend(exclude_patterns.iter().cloned());

    let mut excluded_files = Vec::new();
    let mut binary_files = Vec::new();
    let mut filtered_sections = Vec::new();

    // Split diff into sections (each starting with "diff --git")
    let sections: Vec<&str> = raw_diff.split("diff --git ").collect();

    for (idx, section) in sections.iter().enumerate() {
        if idx == 0 && section.trim().is_empty() {
            // Skip empty first section (before first "diff --git")
            continue;
        }

        // Extract file path from section header
        let file_path = extract_file_path(section);

        // Check if this file should be excluded
        if should_exclude(&file_path, &all_patterns) {
            excluded_files.push(file_path.clone());
            debug!(file = %file_path, "Excluding file from diff");
            continue;
        }

        // Check if this is a binary file
        if is_binary_section(section) {
            binary_files.push(file_path.clone());
            let simplified = simplify_binary_section(&file_path);
            filtered_sections.push(simplified);
            debug!(file = %file_path, "Simplifying binary file");
            continue;
        }

        // Include this section (prepend "diff --git " that was removed by split)
        filtered_sections.push(format!("diff --git {}", section));
    }

    let content = filtered_sections.join("");
    let filtered_size = content.len();

    debug!(
        original_size,
        filtered_size,
        excluded_count = excluded_files.len(),
        binary_count = binary_files.len(),
        "Diff filtering completed"
    );

    FilteredDiff {
        content,
        original_size,
        filtered_size,
        excluded_files,
        binary_files,
    }
}

/// Display warning if the diff size exceeds the threshold
/// Also displays filtering statistics
pub fn warn_if_large(filtered: &FilteredDiff, verbose: bool) {
    if filtered.filtered_size > WARNING_THRESHOLD {
        warn!(
            "Diff is large ({} bytes, {} lines)",
            filtered.filtered_size,
            filtered.line_count()
        );
        eprintln!(
            "⚠ Warning: Diff is large ({} bytes, {} lines)",
            filtered.filtered_size,
            filtered.line_count()
        );
        eprintln!("  Consider splitting into smaller commits.");
    }

    // Display filtering statistics in verbose mode or when there are exclusions
    if verbose || filtered.has_exclusions() {
        if filtered.has_exclusions() {
            eprintln!(
                "ℹ Diff filtering: {} -> {} bytes ({:.1}% reduction)",
                filtered.original_size,
                filtered.filtered_size,
                filtered.reduction_percentage()
            );
        }

        if verbose && !filtered.excluded_files.is_empty() {
            eprintln!("  Excluded files:");
            for file in &filtered.excluded_files {
                eprintln!("    - {}", file);
            }
        }

        if verbose && !filtered.binary_files.is_empty() {
            eprintln!("  Simplified binary files:");
            for file in &filtered.binary_files {
                eprintln!("    - {}", file);
            }
        }
    }
}

/// Extract file path from a diff section
/// Example: "a/Cargo.lock b/Cargo.lock" -> "Cargo.lock"
fn extract_file_path(section: &str) -> String {
    let first_line = section.lines().next().unwrap_or("");

    // Parse "a/path b/path" format
    if let Some(space_pos) = first_line.find(' ') {
        let b_part = &first_line[space_pos + 1..];
        if let Some(path) = b_part.strip_prefix("b/") {
            return path.split_whitespace().next().unwrap_or("").to_string();
        }
    }

    // Fallback: try to extract from "a/path"
    if let Some(path) = first_line.strip_prefix("a/") {
        return path.split_whitespace().next().unwrap_or("").to_string();
    }

    String::new()
}

/// Check if a file path matches any exclusion pattern
fn should_exclude(file_path: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        if matches_pattern(file_path, pattern) {
            return true;
        }
    }
    false
}

/// Simple glob pattern matching
/// Supports: exact match, *.ext, and path/to/*.ext
fn matches_pattern(file_path: &str, pattern: &str) -> bool {
    // Exact match
    if file_path == pattern {
        return true;
    }

    // Wildcard pattern (e.g., "*.lock")
    if pattern.starts_with("*.") {
        let extension = &pattern[1..]; // includes the dot
        return file_path.ends_with(extension);
    }

    // Path with wildcard (e.g., "vendor/*.rs")
    if pattern.contains('*') {
        return simple_glob_match(file_path, pattern);
    }

    false
}

/// Simple glob matching for patterns with wildcards
fn simple_glob_match(path: &str, pattern: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();

    if parts.len() == 1 {
        return path == pattern;
    }

    // Check if path starts with first part
    if !parts[0].is_empty() && !path.starts_with(parts[0]) {
        return false;
    }

    // Check if path ends with last part
    if !parts[parts.len() - 1].is_empty() && !path.ends_with(parts[parts.len() - 1]) {
        return false;
    }

    true
}

/// Check if a diff section represents a binary file
fn is_binary_section(section: &str) -> bool {
    section.contains("Binary files") || section.contains("GIT binary patch")
}

/// Create a simplified representation for a binary file
fn simplify_binary_section(file_path: &str) -> String {
    format!("Binary file {} changed\n", file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================
    // FilteredDiff struct tests
    // =========================================

    #[test]
    fn test_filtered_diff_new() {
        let content = "test content".to_string();
        let original_size = 100;
        let diff = FilteredDiff {
            content: content.clone(),
            original_size,
            filtered_size: content.len(),
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };

        assert_eq!(diff.content, content);
        assert_eq!(diff.original_size, original_size);
        assert_eq!(diff.filtered_size, content.len());
        assert_eq!(diff.excluded_files.len(), 0);
        assert_eq!(diff.binary_files.len(), 0);
    }

    #[test]
    fn test_filtered_diff_line_count() {
        let content = "line1\nline2\nline3".to_string();
        let diff = FilteredDiff {
            content: content.clone(),
            original_size: 100,
            filtered_size: content.len(),
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        assert_eq!(diff.line_count(), 3);
    }

    #[test]
    fn test_filtered_diff_line_count_empty() {
        let diff = FilteredDiff {
            content: String::new(),
            original_size: 0,
            filtered_size: 0,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        assert_eq!(diff.line_count(), 0);
    }

    #[test]
    fn test_filtered_diff_line_count_single_line_no_newline() {
        let diff = FilteredDiff {
            content: "single line".to_string(),
            original_size: 11,
            filtered_size: 11,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        assert_eq!(diff.line_count(), 1);
    }

    #[test]
    fn test_filtered_diff_has_exclusions() {
        let content = "content".to_string();
        let mut diff = FilteredDiff {
            content: content.clone(),
            original_size: 100,
            filtered_size: content.len(),
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        assert!(!diff.has_exclusions());

        diff.excluded_files.push("test.lock".to_string());
        assert!(diff.has_exclusions());
    }

    #[test]
    fn test_filtered_diff_has_exclusions_binary_only() {
        let diff = FilteredDiff {
            content: "content".to_string(),
            original_size: 100,
            filtered_size: 7,
            excluded_files: Vec::new(),
            binary_files: vec!["image.png".to_string()],
        };
        assert!(diff.has_exclusions());
    }

    #[test]
    fn test_filtered_diff_has_exclusions_both() {
        let diff = FilteredDiff {
            content: "content".to_string(),
            original_size: 100,
            filtered_size: 7,
            excluded_files: vec!["Cargo.lock".to_string()],
            binary_files: vec!["image.png".to_string()],
        };
        assert!(diff.has_exclusions());
    }

    #[test]
    fn test_filtered_diff_reduction_percentage() {
        let diff = FilteredDiff {
            content: "test".to_string(),
            original_size: 100,
            filtered_size: 75,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };

        assert_eq!(diff.reduction_percentage(), 25.0);
    }

    #[test]
    fn test_filtered_diff_reduction_percentage_overflow() {
        // Test case where filtered_size > original_size (should return 0.0, not panic)
        let diff = FilteredDiff {
            content: "test".to_string(),
            original_size: 50,
            filtered_size: 100,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };

        assert_eq!(diff.reduction_percentage(), 0.0);

        // Test case where filtered_size == original_size
        let diff2 = FilteredDiff {
            content: "test".to_string(),
            original_size: 100,
            filtered_size: 100,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };

        assert_eq!(diff2.reduction_percentage(), 0.0);
    }

    #[test]
    fn test_filtered_diff_reduction_percentage_zero_original() {
        let diff = FilteredDiff {
            content: String::new(),
            original_size: 0,
            filtered_size: 0,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        assert_eq!(diff.reduction_percentage(), 0.0);
    }

    #[test]
    fn test_filtered_diff_reduction_percentage_full_reduction() {
        let diff = FilteredDiff {
            content: String::new(),
            original_size: 100,
            filtered_size: 0,
            excluded_files: vec!["all.lock".to_string()],
            binary_files: Vec::new(),
        };
        assert_eq!(diff.reduction_percentage(), 100.0);
    }

    #[test]
    fn test_filtered_diff_clone() {
        let original = FilteredDiff {
            content: "test".to_string(),
            original_size: 100,
            filtered_size: 50,
            excluded_files: vec!["file.lock".to_string()],
            binary_files: vec!["image.png".to_string()],
        };
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_filtered_diff_debug() {
        let diff = FilteredDiff {
            content: "test".to_string(),
            original_size: 100,
            filtered_size: 50,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        let debug_str = format!("{:?}", diff);
        assert!(debug_str.contains("FilteredDiff"));
        assert!(debug_str.contains("100"));
        assert!(debug_str.contains("50"));
    }

    #[test]
    fn test_filtered_diff_eq() {
        let diff1 = FilteredDiff {
            content: "test".to_string(),
            original_size: 100,
            filtered_size: 50,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        let diff2 = FilteredDiff {
            content: "test".to_string(),
            original_size: 100,
            filtered_size: 50,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        let diff3 = FilteredDiff {
            content: "different".to_string(),
            original_size: 100,
            filtered_size: 50,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        assert_eq!(diff1, diff2);
        assert_ne!(diff1, diff3);
    }

    // =========================================
    // extract_file_path tests
    // =========================================

    #[test]
    fn test_extract_file_path() {
        let section = "a/Cargo.lock b/Cargo.lock\nindex 1234..5678";
        assert_eq!(extract_file_path(section), "Cargo.lock");

        let section2 = "a/src/main.rs b/src/main.rs\n--- a/src/main.rs";
        assert_eq!(extract_file_path(section2), "src/main.rs");
    }

    #[test]
    fn test_extract_file_path_empty_section() {
        assert_eq!(extract_file_path(""), "");
    }

    #[test]
    fn test_extract_file_path_no_b_prefix() {
        // Fallback to a/ prefix
        let section = "a/only.txt";
        assert_eq!(extract_file_path(section), "only.txt");
    }

    #[test]
    fn test_extract_file_path_invalid_format() {
        let section = "some random text without proper format";
        assert_eq!(extract_file_path(section), "");
    }

    #[test]
    fn test_extract_file_path_deep_nested() {
        let section = "a/src/deeply/nested/path/file.rs b/src/deeply/nested/path/file.rs\n";
        assert_eq!(extract_file_path(section), "src/deeply/nested/path/file.rs");
    }

    #[test]
    fn test_extract_file_path_with_spaces_in_line() {
        // File path extraction stops at whitespace
        let section = "a/file.txt b/file.txt extra stuff\n";
        assert_eq!(extract_file_path(section), "file.txt");
    }

    // =========================================
    // matches_pattern tests
    // =========================================

    #[test]
    fn test_matches_pattern_exact() {
        assert!(matches_pattern("Cargo.lock", "Cargo.lock"));
        assert!(!matches_pattern("Cargo.toml", "Cargo.lock"));
    }

    #[test]
    fn test_matches_pattern_wildcard() {
        assert!(matches_pattern("file.lock", "*.lock"));
        assert!(!matches_pattern("package-lock.json", "*.lock"));
        assert!(matches_pattern("test.lockb", "*.lockb"));
    }

    #[test]
    fn test_matches_pattern_wildcard_extension() {
        assert!(matches_pattern("any.lock", "*.lock"));
        assert!(matches_pattern("deeply/nested/file.lock", "*.lock"));
        assert!(!matches_pattern("file.locked", "*.lock"));
    }

    #[test]
    fn test_matches_pattern_no_match() {
        assert!(!matches_pattern("file.txt", "*.lock"));
        assert!(!matches_pattern("file.txt", "other.txt"));
    }

    // =========================================
    // should_exclude tests
    // =========================================

    #[test]
    fn test_should_exclude_exact_match() {
        let patterns = vec!["Cargo.lock".to_string()];
        assert!(should_exclude("Cargo.lock", &patterns));
        assert!(!should_exclude("Cargo.toml", &patterns));
    }

    #[test]
    fn test_should_exclude_wildcard() {
        let patterns = vec!["*.lock".to_string()];
        assert!(should_exclude("file.lock", &patterns));
        assert!(should_exclude("any.lock", &patterns));
        assert!(!should_exclude("file.txt", &patterns));
    }

    #[test]
    fn test_should_exclude_multiple_patterns() {
        let patterns = vec![
            "Cargo.lock".to_string(),
            "*.json".to_string(),
            "vendor/*".to_string(),
        ];
        assert!(should_exclude("Cargo.lock", &patterns));
        assert!(should_exclude("package.json", &patterns));
        assert!(should_exclude("vendor/lib.rs", &patterns));
        assert!(!should_exclude("src/main.rs", &patterns));
    }

    #[test]
    fn test_should_exclude_empty_patterns() {
        let patterns: Vec<String> = Vec::new();
        assert!(!should_exclude("any_file.txt", &patterns));
    }

    // =========================================
    // simple_glob_match tests
    // =========================================

    #[test]
    fn test_simple_glob_match_no_wildcard() {
        assert!(simple_glob_match("file.txt", "file.txt"));
        assert!(!simple_glob_match("file.txt", "other.txt"));
    }

    #[test]
    fn test_simple_glob_match_prefix_wildcard() {
        // Pattern: *suffix
        assert!(simple_glob_match("test.lock", "*.lock"));
        assert!(simple_glob_match("anything.lock", "*.lock"));
    }

    #[test]
    fn test_simple_glob_match_suffix_wildcard() {
        // Pattern: prefix*
        assert!(simple_glob_match("vendor/lib.rs", "vendor/*"));
        assert!(simple_glob_match("vendor/anything", "vendor/*"));
        assert!(!simple_glob_match("src/lib.rs", "vendor/*"));
    }

    #[test]
    fn test_simple_glob_match_middle_wildcard() {
        // Pattern: prefix*suffix
        assert!(simple_glob_match("test_file.rs", "test*.rs"));
        assert!(simple_glob_match("test_anything_here.rs", "test*.rs"));
        assert!(!simple_glob_match("other_file.rs", "test*.rs"));
    }

    #[test]
    fn test_simple_glob_match_multiple_wildcards() {
        // Multiple wildcards - checks start and end only
        assert!(simple_glob_match("a_b_c", "a*b*c"));
        assert!(simple_glob_match("a_x_c", "a*b*c")); // Only checks first and last parts
    }

    // =========================================
    // is_binary_section tests
    // =========================================

    #[test]
    fn test_is_binary_section() {
        let binary_section = "a/image.png b/image.png\nBinary files differ";
        assert!(is_binary_section(binary_section));

        let text_section = "a/file.txt b/file.txt\n+added line";
        assert!(!is_binary_section(text_section));
    }

    #[test]
    fn test_is_binary_section_git_binary_patch() {
        let section = "a/file.bin b/file.bin\nGIT binary patch\nliteral 1234";
        assert!(is_binary_section(section));
    }

    #[test]
    fn test_is_binary_section_empty() {
        assert!(!is_binary_section(""));
    }

    #[test]
    fn test_is_binary_section_text_mentioning_binary() {
        // This will match because it contains "Binary files" substring
        let section = "// This code handles Binary files";
        assert!(is_binary_section(section));
    }

    // =========================================
    // simplify_binary_section tests
    // =========================================

    #[test]
    fn test_simplify_binary_section() {
        let simplified = simplify_binary_section("image.png");
        assert_eq!(simplified, "Binary file image.png changed\n");
    }

    #[test]
    fn test_simplify_binary_section_nested_path() {
        let simplified = simplify_binary_section("assets/images/logo.png");
        assert_eq!(simplified, "Binary file assets/images/logo.png changed\n");
    }

    #[test]
    fn test_simplify_binary_section_empty_path() {
        let simplified = simplify_binary_section("");
        assert_eq!(simplified, "Binary file  changed\n");
    }

    // =========================================
    // filter_diff tests
    // =========================================

    #[test]
    fn test_filter_diff_excludes_lock_files() {
        let raw_diff = "\
diff --git a/Cargo.lock b/Cargo.lock
index 1234..5678
--- a/Cargo.lock
+++ b/Cargo.lock
+new dependency
diff --git a/src/main.rs b/src/main.rs
index abcd..efgh
--- a/src/main.rs
+++ b/src/main.rs
+fn main() {}
";

        let filtered = filter_diff(raw_diff, &[]);

        assert!(filtered.excluded_files.contains(&"Cargo.lock".to_string()));
        assert!(!filtered.content.contains("Cargo.lock"));
        assert!(filtered.content.contains("src/main.rs"));
    }

    #[test]
    fn test_filter_diff_simplifies_binary() {
        let raw_diff = "\
diff --git a/image.png b/image.png
Binary files differ
diff --git a/src/main.rs b/src/main.rs
--- a/src/main.rs
+++ b/src/main.rs
+code
";

        let filtered = filter_diff(raw_diff, &[]);

        assert!(filtered.binary_files.contains(&"image.png".to_string()));
        assert!(filtered.content.contains("Binary file image.png changed"));
        assert!(!filtered.content.contains("Binary files differ"));
    }

    #[test]
    fn test_filter_diff_with_custom_patterns() {
        let raw_diff = "\
diff --git a/test.json b/test.json
+json content
diff --git a/src/main.rs b/src/main.rs
+rust code
";

        let custom_patterns = vec!["*.json".to_string()];
        let filtered = filter_diff(raw_diff, &custom_patterns);

        assert!(filtered.excluded_files.contains(&"test.json".to_string()));
        assert!(!filtered.content.contains("test.json"));
        assert!(filtered.content.contains("src/main.rs"));
    }

    #[test]
    fn test_filter_diff_empty_input() {
        let filtered = filter_diff("", &[]);
        assert!(filtered.content.is_empty());
        assert_eq!(filtered.original_size, 0);
        assert_eq!(filtered.filtered_size, 0);
        assert!(filtered.excluded_files.is_empty());
        assert!(filtered.binary_files.is_empty());
    }

    #[test]
    fn test_filter_diff_no_exclusions() {
        let raw_diff = "\
diff --git a/src/lib.rs b/src/lib.rs
+pub fn foo() {}
";
        let filtered = filter_diff(raw_diff, &[]);

        assert!(filtered.excluded_files.is_empty());
        assert!(filtered.binary_files.is_empty());
        assert!(filtered.content.contains("src/lib.rs"));
    }

    #[test]
    fn test_filter_diff_all_excluded() {
        let raw_diff = "\
diff --git a/Cargo.lock b/Cargo.lock
+lock content
diff --git a/package-lock.json b/package-lock.json
+json content
";
        let filtered = filter_diff(raw_diff, &[]);

        assert_eq!(filtered.excluded_files.len(), 2);
        assert!(filtered.content.is_empty());
    }

    #[test]
    fn test_filter_diff_preserves_order() {
        let raw_diff = "\
diff --git a/a.rs b/a.rs
+a
diff --git a/b.rs b/b.rs
+b
diff --git a/c.rs b/c.rs
+c
";
        let filtered = filter_diff(raw_diff, &[]);

        let content = &filtered.content;
        let pos_a = content.find("a.rs").unwrap();
        let pos_b = content.find("b.rs").unwrap();
        let pos_c = content.find("c.rs").unwrap();
        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }

    #[test]
    fn test_filter_diff_multiple_default_excludes() {
        let raw_diff = "\
diff --git a/Cargo.lock b/Cargo.lock
+cargo
diff --git a/package-lock.json b/package-lock.json
+npm
diff --git a/yarn.lock b/yarn.lock
+yarn
diff --git a/pnpm-lock.yaml b/pnpm-lock.yaml
+pnpm
diff --git a/src/main.rs b/src/main.rs
+code
";
        let filtered = filter_diff(raw_diff, &[]);

        assert_eq!(filtered.excluded_files.len(), 4);
        assert!(filtered.excluded_files.contains(&"Cargo.lock".to_string()));
        assert!(
            filtered
                .excluded_files
                .contains(&"package-lock.json".to_string())
        );
        assert!(filtered.excluded_files.contains(&"yarn.lock".to_string()));
        assert!(
            filtered
                .excluded_files
                .contains(&"pnpm-lock.yaml".to_string())
        );
        assert!(filtered.content.contains("src/main.rs"));
    }

    // =========================================
    // warn_if_large tests (indirect via filter_diff)
    // =========================================

    #[test]
    fn test_warning_threshold_constant() {
        assert_eq!(WARNING_THRESHOLD, 50 * 1024);
    }

    #[test]
    fn test_warn_if_large_small_diff() {
        let small_diff = FilteredDiff {
            content: "small".to_string(),
            original_size: 5,
            filtered_size: 5,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        // Should not panic or warn
        warn_if_large(&small_diff, false);
    }

    #[test]
    fn test_warn_if_large_with_exclusions_verbose() {
        let diff = FilteredDiff {
            content: "content".to_string(),
            original_size: 1000,
            filtered_size: 7,
            excluded_files: vec!["Cargo.lock".to_string()],
            binary_files: vec!["image.png".to_string()],
        };
        // Should not panic
        warn_if_large(&diff, true);
    }

    #[test]
    fn test_warn_if_large_no_exclusions_verbose() {
        let diff = FilteredDiff {
            content: "content".to_string(),
            original_size: 7,
            filtered_size: 7,
            excluded_files: Vec::new(),
            binary_files: Vec::new(),
        };
        // Should not panic - verbose mode but no exclusions to show
        warn_if_large(&diff, true);
    }

    // =========================================
    // DEFAULT_EXCLUDES constant tests
    // =========================================

    #[test]
    fn test_default_excludes_constant() {
        assert!(DEFAULT_EXCLUDES.contains(&"Cargo.lock"));
        assert!(DEFAULT_EXCLUDES.contains(&"package-lock.json"));
        assert!(DEFAULT_EXCLUDES.contains(&"pnpm-lock.yaml"));
        assert!(DEFAULT_EXCLUDES.contains(&"yarn.lock"));
    }

    #[test]
    fn test_default_excludes_wildcard_patterns() {
        assert!(DEFAULT_EXCLUDES.contains(&"*.lock"));
        assert!(DEFAULT_EXCLUDES.contains(&"*.lockb"));
    }

    #[test]
    fn test_default_excludes_count() {
        // Ensure we have all expected default exclusions
        assert!(DEFAULT_EXCLUDES.len() >= 6);
    }
}
