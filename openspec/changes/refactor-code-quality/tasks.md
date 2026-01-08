# タスク: refactor-code-quality

## 実装タスク

### Phase 1: HTTP クライアント共通化

#### 1.1 共通 HTTP クライアントビルダー関数の作成
- [ ] `src/llm/mod.rs` に `build_http_client()` 関数を追加
- [ ] timeout、connect_timeout を引数として受け取る設計
- [ ] User-Agent 設定を含める
- [ ] `Result<reqwest::Client, JjDescError>` を返す

#### 1.2 openai_compat.rs の更新
- [ ] `new()` メソッド内の HTTP クライアント初期化を共通関数呼び出しに置換
- [ ] 不要なコードを削除
- [ ] 既存テストがパスすることを確認

#### 1.3 anthropic.rs の更新
- [ ] `new()` メソッド内の HTTP クライアント初期化を共通関数呼び出しに置換
- [ ] Anthropic 固有のヘッダー設定は維持
- [ ] 既存テストがパスすることを確認

### Phase 2: テストユーティリティ共通化

#### 2.1 共通 test_config 関数の追加
- [ ] `src/llm/mod.rs` の `#[cfg(test)]` セクションに `pub(crate) fn test_config()` を定義
- [ ] テスト用デフォルト値（provider: OpenAI, model: gpt-4o-mini 等）を設定

#### 2.2 各ファイルでの重複削除
- [ ] `src/llm/openai_compat.rs` のローカル `test_config` を削除し、`super::test_config` を使用
- [ ] `src/llm/anthropic.rs` のローカル `test_config` を削除し、`super::test_config` を使用
- [ ] テストがパスすることを確認

### Phase 3: エラーメッセージ一貫性

#### 3.1 エラーメッセージの監査
- [ ] `src/error.rs` の `JjDescError` 全 variant を確認
- [ ] 小文字開始のメッセージを特定

#### 3.2 エラーメッセージの修正
- [ ] "jj command failed..." を "Jj command failed..." に修正
- [ ] 他に不一致があれば同様に修正

### Phase 4: Config ビルダーパターン統一

#### 4.1 パターン統一
- [ ] `with_max_tokens` の実装を `if let Some(t) = max_tokens` パターンに変更
- [ ] 冗長な `Some()` ラッピングを削除
- [ ] 他のビルダーメソッドも同様のパターンか確認

### Phase 5: マジックナンバー定数化

#### 5.1 定数定義
- [ ] `src/llm/mod.rs` に以下の定数を追加:
  - `DEFAULT_MAX_TOKENS: u32 = 1024`
  - `DEFAULT_TEMPERATURE: f32 = 0.3`
  - `DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30`
  - `DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 5`

#### 5.2 定数の適用
- [ ] `openai_compat.rs` でハードコード値を定数に置換
- [ ] `anthropic.rs` でハードコード値を定数に置換
- [ ] `config.rs` のデフォルト値も定数参照に変更（必要に応じて）

### Phase 6: 検証

#### 6.1 テスト実行
- [ ] `cargo test` 全体の実行確認
- [ ] 各モジュールのテストが個別にパスすることを確認

#### 6.2 CI チェック
- [ ] `cargo clippy -- -D warnings` でエラーなし
- [ ] `cargo fmt --check` でフォーマット確認

## 依存関係

- Phase 1 → Phase 5 で定数を使用するため、Phase 5 を Phase 1 より先に実施することも可
- Phase 2〜4 は独立して実施可能

## 検証方法

1. 全ユニットテストの実行
2. `cargo clippy` と `cargo fmt` のチェック
3. 手動テスト: `jj-desc generate` が正常動作すること
