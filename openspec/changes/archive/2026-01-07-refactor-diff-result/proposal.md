# Proposal: refactor-diff-result

## Why

The current `jj::get_diff()` implementation has several design flaws that create potential bugs and code duplication:

1. **Inconsistent behavior**: When encountering a merge commit with an empty diff, `get_diff()` returns `Ok("")`, which can be passed directly to the LLM in `generate.rs`, leading to unnecessary API calls or unexpected behavior.
2. **Duplicate logic**: Both `generate.rs` and `backfill.rs` independently implement merge commit detection and empty diff handling, violating DRY principles.
3. **Unreachable error branch**: The `Err(EmptyDiff)` branch at `generate.rs:41` is effectively unreachable because `get_diff()` returns `Ok("")` for merge commits internally, making error handling inconsistent.

## What Changes

Introduce a `DiffResult` enum to make `get_diff()` return type more expressive:

```rust
pub enum DiffResult {
    /// Regular diff content
    Content(String),
    /// Merge commit with no changes
    EmptyMerge,
}
```

Change `get_diff()` signature to `Result<DiffResult, JjDescError>` and update callers to handle both variants explicitly.

### Expected Benefits

- **Type safety**: Callers must explicitly handle merge commits, reducing bugs.
- **Code reduction**: Eliminates duplicate merge detection logic in `generate` and `backfill` commands.
- **Clarity**: The return type makes the contract clear at the API level.

### Impact Scope

- `src/jj.rs`: Add `DiffResult` enum, update `get_diff()` return type
- `src/commands/generate.rs`: Match on `DiffResult` variants
- `src/commands/backfill.rs`: Match on `DiffResult` variants

### Priority

**High** — Addresses potential bug in production code path
