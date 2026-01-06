# タスク: refactor-config-resolution

## 実装タスク

### 1. ConfigSource enum の追加
- [x] `src/config.rs` に `ConfigSource` enum を追加
- [x] `Default`, `Environment`, `CommandLine` バリアントを定義

### 2. Config 構造体の拡張
- [x] `model_source: ConfigSource` フィールド追加
- [x] `max_tokens: Option<u32>` フィールド追加
- [x] `temperature: Option<f32>` フィールド追加
- [x] `from_env()` で `model_source` を適切に設定

### 3. with_provider() の修正
- [x] `model_source == Default` の場合のみモデルを置換
- [x] 新 provider の API キーが無い場合はエラーを返す
- [x] 既存テストを更新

### 4. with_model() の修正
- [x] 呼び出し時に `model_source = CommandLine` に設定

### 5. CLI オプションの追加
- [x] `GenerateArgs` に `--max-tokens` 追加
- [x] `GenerateArgs` に `--temperature` 追加
- [x] `BackfillArgs` に同様のオプション追加
- [x] 環境変数 `LLM_MAX_TOKENS`, `LLM_TEMPERATURE` 対応

### 6. openai_compat.rs の更新
- [x] `ChatCompletionRequest` に `max_tokens`, `temperature` 追加
- [x] Config から値を取得してリクエストに含める

### 7. anthropic.rs の更新
- [x] `AnthropicRequest` に `temperature` 追加（max_tokens は既存）
- [x] Config から値を取得

### 8. テストの追加
- [x] `ConfigSource` の追跡が正しく動作するかテスト
- [x] CLI オプションのテスト
- [x] 環境変数のテスト

### 9. CI 確認
- [x] `cargo clippy -- -D warnings` でエラーなし
- [x] `cargo test` 全体の実行確認

## 依存関係

- `refactor-error-types` を先に実装すると、API キー不足エラーの扱いがスムーズ

## 検証方法

1. `--provider openai --model gpt-4o` で provider 切り替えを確認
2. `--max-tokens 500 --temperature 0.7` でパラメータが反映されるか確認
3. 不正な設定時に適切なエラーが出るか確認
