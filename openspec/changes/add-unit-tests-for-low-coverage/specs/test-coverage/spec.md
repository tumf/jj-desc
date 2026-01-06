# Test Coverage Specification

## ADDED Requirements

### Requirement: CLI Argument Parsing Tests

The CLI argument parser (`src/cli.rs`) shall have unit tests covering all subcommands, global options, and validation logic.

#### Scenario: Parse generate subcommand

**Given** the CLI is invoked with `jj-desc generate --revision @`  
**When** the arguments are parsed  
**Then** the `Commands::Generate` variant is matched  
**And** the revision is set to `@`

#### Scenario: Parse backfill subcommand with options

**Given** the CLI is invoked with `jj-desc backfill --revisions 'mutable()' --dry-run`  
**When** the arguments are parsed  
**Then** the `Commands::Backfill` variant is matched  
**And** the revisions are set to `mutable()`  
**And** dry_run is `true`

#### Scenario: Parse global provider option

**Given** the CLI is invoked with `--provider anthropic`  
**When** the arguments are parsed  
**Then** the provider is set to `Provider::Anthropic`

#### Scenario: Parse global model option

**Given** the CLI is invoked with `--model claude-3-5-sonnet-20241022`  
**When** the arguments are parsed  
**Then** the model is set to `"claude-3-5-sonnet-20241022"`

#### Scenario: Reject invalid provider

**Given** the CLI is invoked with `--provider invalid-provider`  
**When** the arguments are parsed  
**Then** an error is returned

---

### Requirement: LLM Client Factory Tests

The LLM client factory (`src/llm/mod.rs`) shall have unit tests verifying correct client instantiation for each provider.

#### Scenario: Create Anthropic client

**Given** a config with `Provider::Anthropic`  
**When** `create_client` is called  
**Then** an `AnthropicClient` is successfully created  
**And** no error is returned

#### Scenario: Create OpenAI client

**Given** a config with `Provider::OpenAi`  
**When** `create_client` is called  
**Then** an `OpenAiCompatClient` is successfully created  
**And** no error is returned

#### Scenario: Create Gemini client

**Given** a config with `Provider::Gemini`  
**When** `create_client` is called  
**Then** an `OpenAiCompatClient` is successfully created  
**And** no error is returned

#### Scenario: Create DeepSeek client

**Given** a config with `Provider::DeepSeek`  
**When** `create_client` is called  
**Then** an `OpenAiCompatClient` is successfully created  
**And** no error is returned

---

### Requirement: Anthropic Client Initialization Tests

The Anthropic client (`src/llm/anthropic.rs`) shall have unit tests verifying proper initialization with various configurations.

#### Scenario: Initialize with default model

**Given** an API key, default model name, and base URL  
**When** `AnthropicClient::new` is called  
**Then** the client is initialized  
**And** the model field matches the provided model name

#### Scenario: Initialize with custom model

**Given** an API key, custom model name, and base URL  
**When** `AnthropicClient::new` is called  
**Then** the client is initialized  
**And** the model field matches the custom model name

#### Scenario: Initialize with custom base URL

**Given** an API key, model name, and custom base URL  
**When** `AnthropicClient::new` is called  
**Then** the client is initialized  
**And** the base_url field matches the custom URL

---

### Requirement: OpenAI-Compatible Client Initialization Tests

The OpenAI-compatible client (`src/llm/openai_compat.rs`) shall have unit tests verifying proper initialization with various configurations.

#### Scenario: Initialize with default model

**Given** an API key, default model name, and base URL  
**When** `OpenAiCompatClient::new` is called  
**Then** the client is initialized  
**And** the model field matches the provided model name

#### Scenario: Initialize with custom model

**Given** an API key, custom model name, and base URL  
**When** `OpenAiCompatClient::new` is called  
**Then** the client is initialized  
**And** the model field matches the custom model name

#### Scenario: Initialize with custom base URL

**Given** an API key, model name, and custom base URL  
**When** `OpenAiCompatClient::new` is called  
**Then** the client is initialized  
**And** the base_url field matches the custom URL

---

### Requirement: Overall Test Coverage Improvement

The overall test coverage shall be increased from 53.66% to at least 60%.

#### Scenario: Measure coverage after tests

**Given** all unit tests are implemented  
**When** `cargo llvm-cov --all-features` is executed  
**Then** the overall line coverage is at least 60%  
**And** `cli.rs` coverage is at least 80%  
**And** `llm/mod.rs` coverage is 100%  
**And** `llm/anthropic.rs` coverage is at least 30%  
**And** `llm/openai_compat.rs` coverage is at least 30%

---

## MODIFIED Requirements

None.

---

## REMOVED Requirements

None.
