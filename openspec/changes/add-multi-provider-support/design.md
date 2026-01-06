# 設計ドキュメント: マルチLLMプロバイダーサポート

## アーキテクチャ概要

```
┌─────────────────────────────────────────────────────────┐
│                      main.rs                            │
│                         │                               │
│                         ▼                               │
│              ┌──────────────────┐                       │
│              │  ProviderConfig  │                       │
│              └────────┬─────────┘                       │
│                       │                                 │
│                       ▼                                 │
│         ┌─────────────────────────┐                     │
│         │   LlmClient (trait)     │                     │
│         └─────────────────────────┘                     │
│                       │                                 │
│    ┌──────────┬───────┼───────┬──────────┐             │
│    ▼          ▼       ▼       ▼          ▼             │
│ ┌──────┐ ┌───────┐ ┌───────┐ ┌──────┐ ┌───────┐       │
│ │OpenAI│ │Anthro │ │Gemini │ │OpenR │ │Custom │       │
│ │Client│ │Client │ │Client │ │outer │ │(将来) │       │
│ └──────┘ └───────┘ └───────┘ └──────┘ └───────┘       │
└─────────────────────────────────────────────────────────┘
```

## 設計判断

### 1. 共通トレイトの採用

**決定**: `LlmClient` トレイトを定義し、各プロバイダーが実装する

**理由**:
- 各プロバイダーの API 差異を吸収
- 将来のプロバイダー追加が容易
- テスト時のモック実装が可能

```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_description(&self, diff: &str) -> Result<String, JjDescError>;
}
```

### 2. プロバイダー種別の列挙型

**決定**: `Provider` 列挙型でプロバイダーを表現

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Provider {
    OpenRouter,
    OpenAI,
    Anthropic,
    Gemini,
}
```

**理由**:
- 型安全なプロバイダー選択
- パターンマッチングによる網羅的な処理
- 無効なプロバイダー名をコンパイル時に検出

### 3. API 互換性の活用

**決定**: OpenAI, Gemini は OpenAI 互換エンドポイントを使用

**理由**:
- コード再利用が可能
- OpenAI, OpenRouter, Gemini で同じリクエスト/レスポンス形式を使用可能
- Anthropic のみ独自形式が必要

### 4. Anthropic Messages API の対応

**決定**: Anthropic は Messages API を使用

**理由**:
- Anthropic は OpenAI 互換 API を提供していない
- Messages API は Anthropic の推奨 API
- リクエスト/レスポンス形式が異なるため専用実装が必要

```rust
// Anthropic Messages API リクエスト形式
#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}
```

### 5. 設定の優先順位

**決定**: 以下の優先順位で設定を解決

1. CLI オプション (`--model`, `--provider`)
2. 環境変数 (`LLM_PROVIDER`, `LLM_MODEL`)
3. プロバイダー固有の環境変数 (`OPENROUTER_MODEL` 等) - 後方互換性
4. デフォルト値

**理由**:
- 既存ユーザーの設定を壊さない（後方互換性）
- CLI オプションが最も明示的なので最優先
- 汎用的な環境変数名で統一感を提供

### 6. デフォルトモデルの設定

| プロバイダー | デフォルトモデル |
|-------------|-----------------|
| OpenRouter | `anthropic/claude-sonnet-4` |
| OpenAI | `gpt-4o` |
| Anthropic | `claude-sonnet-4-20250514` |
| Gemini | `gemini-2.0-flash` |

### 7. カスタムベース URL のサポート

**決定**: 各プロバイダーに対してオプショナルなベース URL 環境変数を提供

| プロバイダー | 環境変数 | デフォルト URL |
|-------------|---------|---------------|
| OpenRouter | `OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` |
| OpenAI | `OPENAI_BASE_URL` | `https://api.openai.com/v1` |
| Anthropic | `ANTHROPIC_BASE_URL` | `https://api.anthropic.com` |
| Gemini | `GEMINI_BASE_URL` | `https://generativelanguage.googleapis.com/v1beta/openai` |

**理由**:
- **Azure OpenAI 対応**: 企業環境では Azure OpenAI Service を使用することが多い
- **セルフホスト対応**: vLLM, Ollama, LocalAI などのセルフホスト LLM サーバーに接続可能
- **プロキシ対応**: 企業のセキュリティポリシーでプロキシ経由のアクセスが必要な場合に対応
- **開発・テスト**: モック API サーバーを使用したテストが容易になる

**ユースケース例**:

```bash
# Azure OpenAI を使用
export LLM_PROVIDER=openai
export OPENAI_API_KEY=your-azure-key
export OPENAI_BASE_URL=https://your-resource.openai.azure.com/openai/deployments/your-deployment

# Ollama (OpenAI 互換モード) を使用
export LLM_PROVIDER=openai
export OPENAI_API_KEY=dummy  # Ollama は API キー不要だが必須フィールド
export OPENAI_BASE_URL=http://localhost:11434/v1

# 企業プロキシ経由で Anthropic を使用
export LLM_PROVIDER=anthropic
export ANTHROPIC_API_KEY=your-key
export ANTHROPIC_BASE_URL=https://proxy.company.com/anthropic
```

**実装詳細**:

```rust
impl Provider {
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "https://openrouter.ai/api/v1",
            Provider::OpenAI => "https://api.openai.com/v1",
            Provider::Anthropic => "https://api.anthropic.com",
            Provider::Gemini => "https://generativelanguage.googleapis.com/v1beta/openai",
        }
    }
    
    pub fn base_url_env_var(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "OPENROUTER_BASE_URL",
            Provider::OpenAI => "OPENAI_BASE_URL",
            Provider::Anthropic => "ANTHROPIC_BASE_URL",
            Provider::Gemini => "GEMINI_BASE_URL",
        }
    }
}
```

## ファイル構造の変更

```
src/
├── llm/
│   ├── mod.rs           # LlmClient トレイトと共通型
│   ├── openai.rs        # OpenAI/OpenRouter/Gemini 用クライアント
│   └── anthropic.rs     # Anthropic 用クライアント
├── config.rs            # 拡張された設定管理
├── provider.rs          # Provider 列挙型と関連ロジック
└── ...
```

## エラーハンドリング

各プロバイダーのエラーレスポンス形式が異なるため、共通のエラー型に変換する：

```rust
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("API request failed: {status} - {message}")]
    ApiError { status: u16, message: String },
    
    #[error("Rate limit exceeded")]
    RateLimited,
    
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Invalid base URL: {0}")]
    InvalidBaseUrl(String),
}
```

## テスト戦略

1. **ユニットテスト**: 各プロバイダークライアントのリクエスト構築をテスト
2. **統合テスト**: モック HTTP サーバーを使用した E2E テスト
3. **カスタム URL テスト**: ベース URL のカスタマイズが正しく動作することをテスト
4. **手動テスト**: 実際の API キーを使用した動作確認

## 移行パス

1. 既存の `OPENROUTER_*` 環境変数は引き続きサポート
2. 新規ユーザーは `LLM_PROVIDER` と `LLM_MODEL` を推奨
3. README に移行ガイドを追加
4. カスタム URL 設定のドキュメントを追加
