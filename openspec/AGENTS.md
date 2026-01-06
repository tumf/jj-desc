# OpenSpec Agent Instructions

## Directory Structure

```
openspec/
├── project.md           # Project overview
├── AGENTS.md            # This file
├── specs/               # Finalized specifications (English)
│   └── <capability>/
│       └── spec.md
└── changes/             # Change proposals (Japanese for draft)
    └── <change-id>/
        ├── proposal.md  # Change proposal summary
        ├── tasks.md     # Implementation tasks
        ├── design.md    # Design decisions (optional)
        └── specs/       # Spec deltas
            └── <capability>/
                └── spec.md
```

## Language Policy

- `openspec/changes/**` — Draft stage: Japanese
- `openspec/specs/**` — Finalized stage: English
- When a change is finalized, translate Japanese drafts to English specs

## Conventions

### Change IDs
- Use verb-led identifiers: `add-feature`, `fix-bug`, `update-component`
- Keep IDs lowercase with hyphens

### Spec Format
```markdown
## ADDED Requirements
#### Requirement: <name>
<description>
#### Scenario: <name>
<steps and expected outcomes>

## MODIFIED Requirements
...

## REMOVED Requirements
...
```

### Task Format
- Ordered list of small, verifiable work items
- Include validation steps (tests, manual checks)
- Mark dependencies explicitly
