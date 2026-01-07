# add-unit-tests-for-low-coverage

## Why

Current test coverage is **53.66%** with several critical modules completely untested:

| Module | Line Coverage | Function Coverage |
|--------|--------------|-------------------|
| `src/cli.rs` | 0% | 0% |
| `src/llm/mod.rs` | 0% (0/5) | 0% (0/1) |
| `src/llm/anthropic.rs` | 0% (0/14) | 0% (0/3) |
| `src/llm/openai_compat.rs` | 0% (0/12) | 0% (0/3) |
| `src/commands/generate.rs` | 0% (0/35) | 0% (0/2) |
| `src/commands/backfill.rs` | 0% (0/99) | 0% (0/2) |
| `src/main.rs` | 0% (0/41) | 0% (0/2) |

Lack of tests for these modules creates significant risk for regression bugs during refactoring or feature additions.

## What Changes

### Phase 1: Add Unit Tests for Core Modules

Priority-ordered test additions:

1. **`src/cli.rs`** (Priority: High)
   - CLI parser tests (subcommands, flags, arguments)
   - Easiest wins with high impact

2. **`src/llm/mod.rs`** (Priority: High)
   - `create_client` provider routing logic
   - Provider initialization tests

3. **`src/llm/anthropic.rs`** (Priority: Medium)
   - Client initialization tests
   - Request body construction tests

4. **`src/llm/openai_compat.rs`** (Priority: Medium)
   - Client initialization tests
   - Request body construction tests

### Phase 2: Out of Scope (Future Consideration)

- `src/commands/generate.rs` - better suited for integration tests
- `src/commands/backfill.rs` - better suited for integration tests
- `src/main.rs` - better suited for E2E tests

### Acceptance Criteria

1. `src/cli.rs` has test cases for:
   - Subcommand parsing (`generate`, `backfill`)
   - Global options (`--provider`, `--model`)
   - Validation (rejecting invalid values)

2. `src/llm/mod.rs` has test cases for:
   - Correct routing for each provider (Anthropic, OpenAI, Gemini, DeepSeek)
   - Unknown provider error handling

3. `src/llm/anthropic.rs` has test cases for:
   - `AnthropicClient::new` initialization
   - `build_request_body` JSON structure

4. `src/llm/openai_compat.rs` has test cases for:
   - `OpenAiCompatClient::new` initialization
   - `build_request_body` JSON structure

5. `cargo llvm-cov` shows coverage **≥60%**

### Impact Scope

- Test code additions only (no production code changes)
- Optional: CI/CD pipeline coverage checks

### Non-Functional Requirements

- Tests execute quickly (no external API calls)
- Appropriate use of mocks and stubs
- Test code is maintainable with clear intent
