# 提案: refactor-diff-result

## 概要

`jj::get_diff()` の戻り値を enum 化し、空 diff とマージコミットの扱いを統一する。

## 背景・動機

現在の実装には以下の問題がある：

1. **不整合な挙動**: `get_diff()` がマージコミット + 空 diff の場合に `Ok("")` を返すが、`generate.rs` 側で空文字列をそのまま LLM に渡してしまう可能性がある
2. **重複ロジック**: `generate.rs` と `backfill.rs` の両方でマージコミット判定と空 diff 処理が個別に実装されている
3. **エラー分岐の到達不能**: `generate.rs:41` の `Err(EmptyDiff)` 分岐が、`get_diff()` 内でマージ判定して `Ok("")` を返す設計のため、実質的に到達しにくい

## 提案内容

`get_diff()` の戻り値を `Result<DiffResult, JjDescError>` に変更：

```rust
pub enum DiffResult {
    /// 通常の diff 内容
    Content(String),
    /// マージコミットで変更なし
    EmptyMerge,
}
```

## 期待される効果

- 呼び出し側が明示的に分岐できるため、バグが減る
- `generate` / `backfill` の重複コードが削減される
- 型システムによる安全性向上

## 影響範囲

- `src/jj.rs`: `DiffResult` enum 追加、`get_diff()` 変更
- `src/commands/generate.rs`: 戻り値の match 対応
- `src/commands/backfill.rs`: 戻り値の match 対応

## 優先度

**高** — 現在のコードにバグの可能性があり、早期対応が望ましい
