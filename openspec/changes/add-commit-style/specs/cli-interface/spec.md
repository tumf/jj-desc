# CLI Interface Spec Delta

## ADDED Requirements

### Requirement: Commit style option

The `jj-desc` command SHALL accept a `--style` option, also settable via the `LLM_COMMIT_STYLE` environment variable, that selects the commit message style. Accepted values are `conventional` and `scoped`. When neither is set, the style SHALL be `conventional`.

#### Scenario: Default style

- **Given** neither `--style` nor `LLM_COMMIT_STYLE` is set
- **When** executing `jj-desc`
- **Then** descriptions are generated in Conventional Commits style

#### Scenario: Style selected via CLI option

- **Given** a commit with an empty description
- **When** executing `jj-desc --style scoped`
- **Then** the description is generated in Scoped Commits style

#### Scenario: Style selected via environment variable

- **Given** `LLM_COMMIT_STYLE=scoped` is set
- **When** executing `jj-desc`
- **Then** the description is generated in Scoped Commits style

#### Scenario: CLI option overrides environment variable

- **Given** `LLM_COMMIT_STYLE=scoped` is set
- **When** executing `jj-desc --style conventional`
- **Then** the description is generated in Conventional Commits style

#### Scenario: Invalid style value

- **When** executing `jj-desc --style invalid`
- **Then** the command fails with an error listing the possible values
