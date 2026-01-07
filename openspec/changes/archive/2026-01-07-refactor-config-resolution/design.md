# 設計: refactor-config-resolution

## 設計判断

### 設定値の優先順位

以下の優先順位で設定値を解決する（高い方が優先）：

1. **CommandLine** — CLI 引数で明示指定
2. **Environment** — 環境変数で設定
3. **Default** — プロバイダごとのデフォルト値

### ConfigSource による追跡

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    Environment,
    CommandLine,
}
```

これにより `with_provider()` で以下の判断が可能：

```rust
pub fn with_provider(mut self, provider: Option<Provider>) -> Result<Self, JjDescError> {
    if let Some(p) = provider {
        self.provider = p;

        // API キーは必ず新 provider 用を取得（なければエラー）
        self.api_key = env::var(p.api_key_env_var())
            .map_err(|_| JjDescError::MissingApiKey)?;

        // モデルはソースに応じて判断
        if self.model_source == ConfigSource::Default {
            // デフォルト値だった場合のみ新 provider のデフォルトに変更
            self.model = p.default_model().to_string();
        }
        // Environment / CommandLine の場合は維持（ユーザーの意図を尊重）

        self.base_url = env::var(p.base_url_env_var())
            .unwrap_or_else(|_| p.default_base_url().to_string());
    }
    Ok(self)
}
```

### LLM パラメータのデフォルト値

| パラメータ | デフォルト | 説明 |
|------------|------------|------|
| `max_tokens` | `1024` | コミットメッセージには十分 |
| `temperature` | `0.3` | 安定した出力を優先 |

### CLI オプション設計

```rust
#[derive(Parser, Debug)]
pub struct GenerateArgs {
    // 既存オプション...

    /// Maximum tokens for LLM response
    #[arg(long, env = "LLM_MAX_TOKENS")]
    pub max_tokens: Option<u32>,

    /// Temperature for LLM response (0.0-2.0)
    #[arg(long, env = "LLM_TEMPERATURE")]
    pub temperature: Option<f32>,
}
```

### API リクエスト構造体の変更

**OpenAI 互換:**
```rust
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}
```

**Anthropic:**
```rust
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,  // Anthropic は必須
    system: String,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}
```

## 後方互換性

- 既存の環境変数は引き続き動作
- 新オプションはすべてオプショナル
- デフォルト動作は現状と同等
