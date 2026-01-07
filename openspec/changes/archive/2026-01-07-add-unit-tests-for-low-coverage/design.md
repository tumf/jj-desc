# Design: Unit Tests for Low Coverage Modules

## テスト戦略

### 1. CLI テスト（`src/cli.rs`）

**アプローチ:** `clap` の derive マクロを使用しているため、パースロジックのテストを行う。

**テスト方法:**
- `Cli::try_parse_from` を使用して引数をパース
- 各フィールドの値が期待通りであることをアサート

**サンプルテスト:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_subcommand() {
        let args = Cli::try_parse_from(&["jj-desc", "generate", "--revision", "@"]).unwrap();
        assert!(matches!(args.command, Some(Commands::Generate { .. })));
    }

    #[test]
    fn test_global_options() {
        let args = Cli::try_parse_from(&[
            "jj-desc",
            "--provider", "anthropic",
            "--model", "claude-3-5-sonnet-20241022"
        ]).unwrap();
        assert_eq!(args.provider, Some(Provider::Anthropic));
        assert_eq!(args.model, Some("claude-3-5-sonnet-20241022".to_string()));
    }
}
```

**依存関係:** なし（`clap` 自体がテスト機能を提供）

---

### 2. LLM モジュールテスト（`src/llm/mod.rs`）

**アプローチ:** `create_client` 関数のプロバイダー振り分けロジックをテスト。

**テスト方法:**
- 各プロバイダーに対して `create_client` を呼び出し
- 返されるクライアントの型を確認（trait object なので直接型チェックはできない）
- 初期化が成功することを確認（`Result::Ok` が返る）

**サンプルテスト:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_anthropic_client() {
        let config = Config::default().with_provider(Provider::Anthropic);
        let result = create_client(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_openai_client() {
        let config = Config::default().with_provider(Provider::OpenAi);
        let result = create_client(&config);
        assert!(result.is_ok());
    }
}
```

**依存関係:** `Config` の初期化（既存のテストで検証済み）

---

### 3. Anthropic クライアントテスト（`src/llm/anthropic.rs`）

**アプローチ:** クライアント初期化とリクエストボディ構築のテスト。

**課題:**
- `generate_description` は非同期でHTTPリクエストを送信するため、ユニットテストには不適切
- 代わりに、リクエストボディの構築ロジックをテスト可能にする

**解決策:**
1. `build_request_body` のような内部関数を `pub(crate)` または `#[cfg(test)]` で公開
2. JSON構造が正しいことをテスト

**サンプルテスト:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_initialization() {
        let client = AnthropicClient::new(
            "test-key".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            "https://api.anthropic.com".to_string(),
        );
        // 初期化が成功することを確認
        assert_eq!(client.model, "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_request_body_structure() {
        // リクエストボディ構築のロジックをテスト
        // 実際の実装に応じて調整が必要
    }
}
```

**代替案:** モックHTTPサーバーを使用（`wiremock` crate）
- 利点: 実際のHTTPリクエストをテストできる
- 欠点: テストが遅くなる、依存関係が増える
- **判断:** フェーズ1では不採用、フェーズ2で検討

---

### 4. OpenAI互換クライアントテスト（`src/llm/openai_compat.rs`）

**アプローチ:** Anthropicクライアントと同様の戦略。

**サンプルテスト:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_initialization() {
        let client = OpenAiCompatClient::new(
            "test-key".to_string(),
            "gpt-4".to_string(),
            "https://api.openai.com/v1".to_string(),
        );
        assert_eq!(client.model, "gpt-4");
    }
}
```

---

## テストツール選定

### 既存の依存関係

- `rstest`: パラメータ化テストに使用中
- `tokio`: 非同期ランタイム（テストで `#[tokio::test]` を使用）

### 追加検討するツール

| ツール | 用途 | 採用判断 |
|--------|------|---------|
| `wiremock` | HTTPモックサーバー | フェーズ1では不採用 |
| `assert_cmd` | CLIテスト | 統合テストで検討 |
| `mockito` | HTTPモック | フェーズ1では不採用 |

**結論:** 既存の `rstest` と標準ライブラリのみで十分。

---

## カバレッジ目標

### 現状（変更前）

| モジュール | カバレッジ |
|-----------|----------|
| 全体 | 53.66% |
| `cli.rs` | 0% |
| `llm/mod.rs` | 0% |
| `llm/anthropic.rs` | 0% |
| `llm/openai_compat.rs` | 0% |

### 目標（変更後）

| モジュール | 目標カバレッジ | 根拠 |
|-----------|------------|------|
| 全体 | 60%以上 | +7%の向上 |
| `cli.rs` | 80%以上 | パースロジック中心 |
| `llm/mod.rs` | 100% | 5行のみ、完全カバー可能 |
| `llm/anthropic.rs` | 30%以上 | 初期化のみテスト |
| `llm/openai_compat.rs` | 30%以上 | 初期化のみテスト |

**注:** LLMクライアントの `generate_description` は統合テストで扱うため、ユニットテストではカバーしない。

---

## リスクと緩和策

### リスク1: テストが外部依存を持つ

**緩和策:** 環境変数やHTTPクライアントをモック化、またはテスト時にダミー値を使用

### リスク2: 非同期処理のテストが複雑

**緩和策:** 非同期処理を含まないロジック（初期化、データ構築）に焦点を当てる

### リスク3: テスト追加によるビルド時間増加

**緩和策:** ユニットテストは高速に実行されるため、影響は最小限

---

## 実装順序の根拠

1. **`cli.rs`**: 最も簡単で効果が高い（純粋な値のパース）
2. **`llm/mod.rs`**: 5行のみで完全カバーが容易
3. **`llm/anthropic.rs`**, **`llm/openai_compat.rs`**: 並行実装可能、類似構造
