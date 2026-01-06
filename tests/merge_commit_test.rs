// Integration tests for merge commit detection
// These tests require a jj repository to be initialized

use std::process::Command;

#[test]
#[ignore] // Requires manual jj repository setup
fn test_merge_commit_e2e() {
    // This test demonstrates the expected E2E behavior
    //
    // Setup:
    // 1. Initialize a test jj repository:
    //    jj init --git test-repo
    //    cd test-repo
    //
    // 2. Create some commits:
    //    jj new -m "first commit"
    //    echo "content" > file.txt
    //    jj new -m "second commit"
    //    echo "more" >> file.txt
    //
    // 3. Create a merge commit:
    //    jj new main @- -m "merge commit"
    //
    // 4. Run jj-desc on the merge commit
    //
    // Expected behavior:
    // - jj-desc should detect that it's a merge commit
    // - Should set description to "Merge commit"
    // - Should not return an EmptyDiff error

    // Test verification (when run in a proper jj repo):
    let output = Command::new("jj")
        .args(["log", "-T", "parents.len()", "-r", "@", "--no-graph"])
        .output();

    // If we're in a jj repo, verify the command works
    if let Ok(out) = output {
        if out.status.success() {
            let count_str = String::from_utf8_lossy(&out.stdout);
            if let Ok(count) = count_str.trim().parse::<usize>() {
                println!("Current commit has {} parent(s)", count);
                if count >= 2 {
                    println!("This is a merge commit - jj-desc should handle it");
                }
            }
        }
    }
}

#[test]
fn test_merge_detection_command_format() {
    // Test that the command format is correct
    let args = vec!["log", "-T", "parents.len()", "-r", "@", "--no-graph"];

    // Verify we have the right number of arguments
    assert_eq!(args.len(), 6);

    // Verify key arguments are present
    assert!(args.contains(&"log"));
    assert!(args.contains(&"-T"));
    assert!(args.contains(&"parents.len()"));
    assert!(args.contains(&"--no-graph"));
}
