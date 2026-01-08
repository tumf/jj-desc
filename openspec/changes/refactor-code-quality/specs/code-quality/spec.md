# Code Quality Spec Delta

## MODIFIED Requirements

### Requirement: DRY HTTP Client Initialization

The LLM module MUST provide a shared HTTP client builder function to eliminate code duplication between OpenAI-compatible and Anthropic clients.

#### Scenario: Shared HTTP client builder is used by all LLM clients

- **Given** the jj-desc codebase
- **When** a developer needs to create an HTTP client for any LLM provider
- **Then** they should use the common `build_http_client()` function from `llm/mod.rs`
- **And** the function should apply consistent timeout, user-agent, and TLS settings

### Requirement: Centralized Test Utilities

The test modules MUST use a shared `test_config()` helper function instead of duplicating the same helper in multiple files.

#### Scenario: Test config helper is shared across LLM tests

- **Given** tests in `llm/mod.rs`, `llm/openai_compat.rs`, and `llm/anthropic.rs`
- **When** a test needs a mock Config instance
- **Then** it should use the shared `test_config()` function from `llm/mod.rs`
- **And** individual test files should not define their own `test_config()` functions

### Requirement: Consistent Error Message Formatting

All error messages in `JjDescError` MUST use consistent capitalization (sentence case with uppercase first letter).

#### Scenario: Error messages have consistent capitalization

- **Given** the `JjDescError` enum in `error.rs`
- **When** any error variant is displayed
- **Then** the message should start with an uppercase letter
- **And** follow sentence case formatting throughout

### Requirement: Unified Config Builder Pattern

All `Config` builder methods MUST use the same `if let Some(x) = opt` pattern for Option handling.

#### Scenario: Config builder methods use consistent pattern

- **Given** the `Config` struct with builder methods (`with_model`, `with_max_tokens`, etc.)
- **When** implementing or modifying builder methods
- **Then** they should use `if let Some(x) = opt { ... }` pattern
- **And** avoid redundant `if opt.is_some() { self.field = opt }` patterns

### Requirement: Named Constants for Configuration Values

Magic numbers related to LLM configuration MUST be defined as named constants.

#### Scenario: Default LLM parameters use named constants

- **Given** default values for `max_tokens`, `temperature`, `timeout`, and `connect_timeout`
- **When** these values are used in client initialization
- **Then** they should reference named constants from `llm/mod.rs`
- **And** the constants should be documented with their purpose
