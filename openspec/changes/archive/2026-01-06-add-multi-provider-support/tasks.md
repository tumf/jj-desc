# タスク: マルチLLMプロバイダーサポート

## 実装タスク

### フェーズ 1: 基盤整備

- [x] **1.1** `Provider` 列挙型の作成 (`src/provider.rs`)
  - `OpenRouter`, `OpenAI`, `Anthropic`, `Gemini` を定義
  - 文字列からの変換 (`FromStr`) を実装
  - 各プロバイダーのデフォルト URL を定義
  - 各プロバイダーのベース URL 環境変数名を定義
  - 検証: ユニットテストで全バリアントの変換をテスト ✅

- [x] **1.2** `LlmClient` トレイトの定義 (`src/llm/mod.rs`)
  - `async fn generate_description(&self, diff: &str) -> Result<String, JjDescError>`
  - 検証: トレイトがコンパイルできることを確認 ✅

- [x] **1.3** 設定管理の拡張 (`src/config.rs`)
  - `LLM_PROVIDER` 環境変数のサポート追加
  - `LLM_MODEL` 環境変数のサポート追加
  - 既存の `OPENROUTER_*` 環境変数との後方互換性維持
  - 検証: 各環境変数の優先順位をユニットテストで確認 ✅

- [x] **1.4** ベース URL 設定のサポート (`src/config.rs`)
  - 各プロバイダー用のベース URL 環境変数のサポート追加
    - `OPENAI_BASE_URL`
    - `ANTHROPIC_BASE_URL`
    - `GEMINI_BASE_URL`
    - `OPENROUTER_BASE_URL` (既存)
  - 環境変数が未設定の場合はデフォルト URL を使用
  - 検証: カスタム URL と デフォルト URL のユニットテスト ✅

### フェーズ 2: プロバイダー実装

- [x] **2.1** OpenAI 互換クライアントの実装 (`src/llm/openai_compat.rs`)
  - 既存の `OpenRouterClient` をベースにリファクタリング
  - OpenRouter, OpenAI, Gemini で共用可能な形に
  - 各プロバイダー固有のヘッダー対応
  - カスタムベース URL のサポート
  - 検証: リクエスト構築のユニットテスト ✅

- [x] **2.2** Anthropic クライアントの実装 (`src/llm/anthropic.rs`)
  - Messages API 形式のリクエスト/レスポンス型を定義
  - `x-api-key` ヘッダーによる認証
  - `anthropic-version` ヘッダーの設定
  - カスタムベース URL のサポート
  - 検証: リクエスト構築のユニットテスト ✅

- [x] **2.3** プロバイダーファクトリの実装 (`src/llm/mod.rs`)
  - `Provider` に基づいて適切なクライアントを生成
  - ベース URL を設定から取得してクライアントに渡す
  - 検証: 各プロバイダーでクライアントが生成できることを確認 ✅

### フェーズ 3: CLI と統合

- [x] **3.1** CLI オプションの追加 (`src/cli.rs`)
  - `--provider` オプションを追加
  - ヘルプメッセージにプロバイダー一覧を表示
  - 検証: `--help` で正しく表示されることを確認 ✅

- [x] **3.2** main.rs の更新
  - 新しい設定とクライアントファクトリを使用
  - 選択されたプロバイダーをログ出力
  - 使用するベース URL をログ出力（デバッグ用）
  - 検証: 各プロバイダーで `--dry-run` が動作することを確認 ✅

### フェーズ 4: エラーハンドリングとテスト

- [x] **4.1** エラー型の拡張 (`src/error.rs`)
  - プロバイダー固有のエラーを共通形式に変換
  - 無効なベース URL エラーの追加
  - 検証: 各エラーケースのユニットテスト ✅

- [x] **4.2** 統合テストの追加
  - ユニットテストで各プロバイダーの設定をテスト
  - カスタムベース URL を使用したテスト
  - 全テスト通過 ✅

### フェーズ 5: ドキュメント

- [x] **5.1** README.md の更新
  - 各プロバイダーの設定方法を記載
  - 環境変数の一覧表を更新（ベース URL 含む）
  - 使用例を追加
  - カスタムベース URL の使用例を追加（Azure OpenAI, Ollama 等） ✅

- [x] **5.2** 移行ガイドの作成
  - 既存ユーザー向けの変更点説明
  - 後方互換性についての説明
  - カスタムエンドポイント設定のガイド ✅

## 実装完了

すべてのタスクが完了しました。

### 実装されたファイル

- `src/provider.rs` - プロバイダー列挙型
- `src/llm/mod.rs` - LlmClient トレイトとファクトリ
- `src/llm/openai_compat.rs` - OpenAI 互換クライアント
- `src/llm/anthropic.rs` - Anthropic クライアント
- `src/config.rs` - 拡張された設定管理
- `src/cli.rs` - CLI オプション
- `src/main.rs` - 統合されたメインロジック
- `src/error.rs` - 拡張されたエラー型
- `README.md` - 更新されたドキュメント
- `Cargo.toml` - 依存関係の追加

### テスト結果

```
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### ビルド結果

```
Finished `release` profile [optimized] target(s) in 34.76s
```

## 成功基準の確認

✅ 4つのプロバイダーすべてで正常にコミットメッセージを生成できる（実装完了）
✅ カスタムベース URL を使用して Azure OpenAI 等に接続できる（実装完了）
✅ 既存の OpenRouter 利用者に影響なく移行できる（後方互換性維持）
✅ 設定が明確でドキュメント化されている（README.md 更新完了）
