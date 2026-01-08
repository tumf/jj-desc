# 設計: refactor-code-quality

## 設計判断

### 1. HTTP クライアント共通化の方針

#### 検討した選択肢

| 方針 | 概要 | 評価 |
|------|------|------|
| A: 関数抽出 | `build_http_client()` 関数を `llm/mod.rs` に追加 | ✅ 採用 |
| B: struct 化 | `HttpClientConfig` struct + `build()` メソッド | △ 過剰な抽象化 |
| C: trait 定義 | `HttpClientBuilder` trait | ✗ 2実装のみで不要 |

#### 採用理由（方針A）

- 最小限の変更で重複を排除できる
- 追加の型定義が不要
- 必要に応じて将来的に struct 化への拡張が容易

#### 設計詳細

```rust
// src/llm/mod.rs

/// HTTP クライアントの共通設定を適用してビルドする
pub fn build_http_client(
    timeout_secs: u64,
    connect_timeout_secs: u64,
) -> Result<reqwest::Client, JjDescError> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .connect_timeout(std::time::Duration::from_secs(connect_timeout_secs))
        .user_agent(format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| JjDescError::HttpClientError(e.to_string()))
}
```

### 2. テストユーティリティの配置

#### 検討した選択肢

| 方針 | 概要 | 評価 |
|------|------|------|
| A: llm/mod.rs 内 | `#[cfg(test)]` ブロック内に `pub(crate)` で定義 | ✅ 採用 |
| B: tests/common.rs | 統合テスト用の共通モジュール | △ 単体テストでは使いにくい |
| C: src/test_utils.rs | 専用モジュール | △ 現時点では過剰 |

#### 採用理由（方針A）

- 現在の3箇所はすべて `src/llm/` 内
- `pub(crate)` で crate 内からのみアクセス可能
- 将来的に共通化範囲が広がれば tests/common.rs への移動を検討

```rust
// src/llm/mod.rs

#[cfg(test)]
pub(crate) fn test_config() -> Config {
    Config {
        provider: Provider::OpenAI,
        model: "gpt-4o-mini".to_string(),
        api_key: "test-key".to_string(),
        api_endpoint: None,
        max_tokens: 1024,
        temperature: 0.3,
    }
}
```

### 3. エラーメッセージ規約

#### 採用規約

Rust の慣例に従い、エラーメッセージは**小文字開始**とする。

> **注**: Rust の標準ライブラリおよび主要クレートでは、エラーメッセージは小文字で開始することが推奨されている。

しかし、現在のコードベースでは大文字開始が多数派であるため、**既存コードとの一貫性を優先**し、大文字開始に統一する。

```rust
// 修正例
#[error("Jj command failed: {0}")]  // "jj" → "Jj"
JjCommandFailed(String),
```

### 4. Config ビルダーパターン

#### 統一パターン

```rust
// 推奨パターン
pub fn with_model(mut self, model: Option<String>) -> Self {
    if let Some(m) = model {
        self.model = m;
    }
    self
}

// 非推奨（現在の with_max_tokens の実装）
pub fn with_max_tokens(mut self, max_tokens: Option<u32>) -> Self {
    if max_tokens.is_some() {
        self.max_tokens = max_tokens;  // 冗長: すでに Option
    }
    self
}
```

#### 修正後

```rust
pub fn with_max_tokens(mut self, max_tokens: Option<u32>) -> Self {
    if let Some(t) = max_tokens {
        self.max_tokens = Some(t);
    }
    self
}
```

### 5. 定数の命名規約

#### 命名規則

- SCREAMING_SNAKE_CASE を使用
- プレフィックスで用途を明示: `DEFAULT_`, `MAX_`, `MIN_`

#### 定数一覧

```rust
// src/llm/mod.rs

/// LLM レスポンスの最大トークン数のデフォルト値
pub const DEFAULT_MAX_TOKENS: u32 = 1024;

/// LLM リクエストの temperature のデフォルト値
pub const DEFAULT_TEMPERATURE: f32 = 0.3;

/// HTTP リクエストのタイムアウト（秒）
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// HTTP 接続のタイムアウト（秒）
pub const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 5;
```

## 後方互換性

- 外部 API への影響: なし（CLI ツールのため）
- 内部 API への影響: 関数シグネチャ変更なし
- 動作結果: 同一（リファクタリングのみ）

## 検討したが採用しなかった案

### HTTP クライアントの遅延初期化 (lazy_static)

```rust
// 検討案
lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = build_http_client().unwrap();
}
```

**却下理由**: エラーハンドリングが困難、テスト時のモック化が複雑になる

### Builder パターンの全面導入

**却下理由**: 現在の `Config::from_env().with_*()` チェーンで十分機能している
