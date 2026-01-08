# Change Proposal: Skip Empty Non-Merge Commits

## Overview

Currently, the tool automatically sets a placeholder description `(empty commit)` for non-merge commits with empty diffs. This behavior will be discontinued, and empty non-merge commits will be skipped instead.

## Background

- Empty merge commits are normal in jj's specification, and setting the description `Merge branches` is appropriate
- However, when a non-merge commit is empty, it represents a "commit that does nothing," and setting a placeholder description has little value
- Rather, detecting empty commits and notifying the user is more useful

## Changes

1. **Behavior Change**: When an empty non-merge commit is detected, skip processing without setting a description
2. **User Notification**: Display a console message when skipping
3. **Constant Removal**: Remove the `EMPTY_NON_MERGE_DESCRIPTION` constant

## Impact Scope

- `src/jj.rs`: Remove `EMPTY_NON_MERGE_DESCRIPTION` constant
- `src/commands/mod.rs`: Change handling of `EmptyNonMerge` case (to skip)
- `openspec/specs/merge-detection/spec.md`: Update specification

## Related Materials

- Current specification: `openspec/specs/merge-detection/spec.md`
