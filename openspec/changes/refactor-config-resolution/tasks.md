# タスク: refactor-config-resolution

## 実装タスク

### 1. ConfigSource enum の追加
- [ ] `src/config.rs` に `ConfigSource` enum を追加
- [ ] `Default`, `Environment`, `CommandLine` バリアントを定義

### 2. Config 構造体の拡張
- [ ] `model_source: ConfigSource` フィールド追加
- [ ] `max_tokens: Option<u32>` フィールド追加
- [ ] `temperature: Option<f32>` フィールド追加
- [ ] `from_env()` で `model_source` を適切に設定

### 3. with_provider() の修正
- [ ] `model_source == Default` の場合のみモデルを置換
- [ ] 新 provider の API キーが無い場合はエラーを返す
- [ ] 既存テストを更新

### 4. with_model() の修正
- [ ] 呼び出し時に `model_source = CommandLine` に設定

### 5. CLI オプションの追加
- [ ] `GenerateArgs` に `--max-tokens` 追加
- [ ] `GenerateArgs` に `--temperature` 追加
- [ ] `BackfillArgs` に同様のオプション追加
- [ ] 環境変数 `LLM_MAX_TOKENS`, `LLM_TEMPERATURE` 対応

### 6. openai_compat.rs の更新
- [ ] `ChatCompletionRequest` に `max_tokens`, `temperature` 追加
- [ ] Config から値を取得してリクエストに含める

### 7. anthropic.rs の更新
- [ ] `AnthropicRequest` に `temperature` 追加（max_tokens は既存）
- [ ] Config から値を取得

### 8. テストの追加
- [ ] `ConfigSource` の追跡が正しく動作するかテスト
- [ ] CLI オプションのテスト
- [ ] 環境変数のテスト

### 9. CI 確認
- [ ] `cargo clippy -- -D warnings` でエラーなし
- [ ] `cargo test` 全体の実行確認

## 依存関係

- `refactor-error-types` を先に実装すると、API キー不足エラーの扱いがスムーズ

## 検証方法

1. `--provider openai --model gpt-4o` で provider 切り替えを確認
2. `--max-tokens 500 --temperature 0.7` でパラメータが反映されるか確認
3. 不正な設定時に適切なエラーが出るか確認
