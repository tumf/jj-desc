# LLM Integration Specification

## ADDED Requirements

### Requirement: OpenRouter API Configuration

The tool MUST read LLM configuration from environment variables.

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENROUTER_API_KEY` | Yes | - | API key for OpenRouter |
| `OPENROUTER_MODEL` | No | `anthropic/claude-sonnet-4-5` | LLM model to use |

#### Scenario: API key is set

**Given** `OPENROUTER_API_KEY` is set to a valid key
**When** the tool initializes
**Then** configuration loads successfully

#### Scenario: API key is missing

**Given** `OPENROUTER_API_KEY` is not set
**When** the tool initializes
**Then** an error message is displayed: "Error: OPENROUTER_API_KEY environment variable is not set"
**And** the tool exits with code 1

#### Scenario: Custom model via environment variable

**Given** `OPENROUTER_MODEL` is set to `openai/gpt-4o`
**When** the tool generates a description
**Then** the specified model is used

#### Scenario: Default model when not specified

**Given** `OPENROUTER_MODEL` is not set
**When** the tool generates a description
**Then** `anthropic/claude-sonnet-4-5` is used

---

### Requirement: OpenRouter API Integration

The tool MUST integrate with OpenRouter's Chat Completions API.

**Endpoint:** `https://openrouter.ai/api/v1/chat/completions`

**Request Headers:**
- `Authorization: Bearer {OPENROUTER_API_KEY}`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "model": "{model}",
  "messages": [
    {"role": "system", "content": "{system_prompt}"},
    {"role": "user", "content": "{user_prompt}"}
  ]
}
```

#### Scenario: Successful API call

**Given** valid API credentials and diff content
**When** the tool calls the OpenRouter API
**Then** the generated description is extracted from the response

#### Scenario: API returns error

**Given** the API returns an error response
**When** the tool processes the response
**Then** an appropriate error message is displayed
**And** the tool exits with code 1

#### Scenario: Network timeout

**Given** the API does not respond within 30 seconds
**When** the timeout is reached
**Then** a timeout error message is displayed
**And** the tool exits with code 1

---

### Requirement: Prompt Design

The tool MUST use an effective prompt to generate meaningful commit descriptions.

**System Prompt:**
```
You are a helpful assistant that generates concise git commit descriptions.
Analyze the provided diff and generate a clear, meaningful commit message.
```

**User Prompt Template:**
```
Generate a commit message for the following diff:

<diff>
{diff_content}
</diff>

Requirements:
- Use imperative mood (e.g., "Add", "Fix", "Update")
- First line should not exceed 72 characters
- Be concise but descriptive
- Focus on the "why" and "what", not the "how"
```

#### Scenario: Generate description from diff

**Given** a diff showing addition of a new function
**When** the prompt is sent to the LLM
**Then** the response contains a commit message in imperative mood

#### Scenario: Handle large diffs

**Given** a diff exceeding 10,000 characters
**When** the tool processes the diff
**Then** the diff is included in full (no truncation)
**And** the LLM generates a summary description
