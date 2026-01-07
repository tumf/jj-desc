# Design: unify-cli-subcommands

## Current Structure

```
jj-desc
├── generate     # GenerateArgs
│   ├── --dry-run
│   ├── --provider
│   ├── --model
│   ├── --max-tokens
│   ├── --temperature
│   └── --revision      # Single revision (optional)
│
└── backfill     # BackfillArgs
    ├── --dry-run
    ├── --provider
    ├── --model
    ├── --max-tokens
    ├── --temperature
    ├── --revisions     # revset (default: ::@ & mutable())
    ├── --limit
    └── --interactive
```

## Structure After Unification

```
jj-desc
├── --dry-run
├── --provider
├── --model
├── --max-tokens
├── --temperature
├── -r/--revisions     # revset (default: ::@ & mutable())
├── -n/--limit
└── -i/--interactive
```

## Design Decisions

### 1. Unifying Revision Specification

**Decision**: Unify `--revision` and `--revisions` to `-r/--revisions`

**Rationale**:
- In jj, `-r` is the standard pattern for accepting revsets
- Single commits (e.g., `@`) are valid revsets
- Users no longer need to think about "single vs. multiple"

### 2. Default Behavior

**Decision**: Default to `::@ & mutable()` processing only commits with empty descriptions

**Rationale**:
- Covers the most common use case (filling descriptions for work-in-progress commits)
- Compatible with existing backfill default behavior
- Commits with existing descriptions are automatically skipped

### 3. Removing Single-Commit Enforcement

**Decision**: Remove the "single commit only" constraint from `generate`

**Rationale**:
- Single commits can be specified with `-r @`
- Processing count can be limited with `-n 1`
- No need for special constraints

### 4. Unifying Command Implementation

**Decision**: Unify based on `execute_backfill`, remove `execute_generate`

**Rationale**:
- Backfill logic subsumes generate logic
- Reduces code duplication
- Simplifies testing

## Migration Plan

1. Change CLI definition (`cli.rs`)
2. Unify command execution logic (`commands/mod.rs`)
3. Update tests
4. Update documentation

## Alternative Approaches Considered

### Option A: Alias Approach
Keep `generate` as an alias for `backfill -n 1`
→ Rejected: Complexity remains

### Option B: New `describe` Subcommand
Add a unified subcommand `jj-desc describe`
→ Rejected: Subcommand itself is redundant
