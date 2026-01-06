# Proposal: Merge Commit Detection and Description Setting

## Change ID
`add-merge-commit-description`

## Why
Currently, `jj-desc` returns an error when `jj diff` output is empty. However, in jj, merge commits are often treated as "empty" (from FAQ: "merge commits are often empty"). We need to enable setting an appropriate description for merge commits even when they have no diff.

## What Changes
Add functionality to detect merge commits and set appropriate descriptions when `jj diff` is empty:

1. Retrieve the number of parents for a commit using `jj log -T 'parents.len()' -r <rev> --no-graph`
2. Determine if it's a merge commit (2 or more parents)
3. Generate an appropriate description like "Merge commit" for merge commits

### Scope
- `src/jj.rs`: Add merge commit detection function
- `src/main.rs`: Modify flow for handling empty diffs

## Related Documentation
- [jj FAQ - Why are most merge commits marked as "empty"?](https://docs.jj-vcs.dev/latest/FAQ/#why-are-most-merge-commits-marked-as-empty)
