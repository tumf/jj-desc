# タスク: refactor-error-types

## 実装タスク

### 1. 新しいエラーバリアントの追加
- [x] `src/error.rs` に `ApiStatus { status: u16, body: String }` を追加
- [x] `ApiResponseError(String)` を追加（レスポンスパースエラー用）
- [x] 既存のテストを更新

### 2. openai_compat.rs の修正
- [x] HTTP エラー時に `JjDescError::ApiStatus` を使用
- [x] レスポンスパースエラー時に `ApiResponseError` を使用
- [x] `JjCommand` の使用箇所を削除

### 3. anthropic.rs の修正
- [x] HTTP エラー時に `JjDescError::ApiStatus` を使用
- [x] レスポンスパースエラー時に `ApiResponseError` を使用
- [x] `JjCommand` の使用箇所を削除

### 4. config.rs の Provider パース修正
- [x] `LLM_PROVIDER` が存在してパース不能な場合、`InvalidProvider` を返す
- [x] 環境変数が未設定の場合のみ `OpenRouter` にフォールバック
- [x] テストケースを追加

### 5. エラーメッセージのテスト
- [x] 新バリアントの `Display` 実装テスト
- [x] エラーメッセージが適切か確認

### 6. CI 確認
- [x] `cargo clippy -- -D warnings` でエラーなし
- [x] `cargo test` 全体の実行確認

## 依存関係

なし（独立して実装可能）

## 検証方法

1. 不正な `LLM_PROVIDER` を設定して適切なエラーが出るか確認
2. 無効な API キーで API エラーメッセージを確認
3. ユニットテストの追加・実行
