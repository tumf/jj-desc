# backfill-empty-descriptions

## 概要

過去に遡って description が設定されていないコミットに対して、LLM を使用して description を自動生成・設定する機能を追加する。

## 背景

現在の `jj-desc` は単一のコミット（デフォルトで `@`）に対してのみ description を生成する。しかし、以下のようなケースで過去のコミットにも description を設定したいニーズがある：

- 既存のリポジトリで `jj-desc` を導入する際、過去の空 description コミットを一括処理したい
- 作業中に description を省略していたコミット群を後からまとめて整理したい
- レビュー前に複数コミットの description を一括生成したい

## 主要機能

### 新コマンド: `jj-desc backfill`

過去のコミットに対して一括で description を生成・設定する。

**主要オプション:**
- `--revisions` / `-r`: 対象範囲を jj revset で指定（デフォルト: `mutable()`）
- `--dry-run`: 実際に設定せず、プレビューのみ
- `--limit` / `-n`: 処理するコミット数の上限
- `--interactive` / `-i`: 各コミットごとに確認を求める
- `--verbose` / `-v`: 詳細ログを出力

### CLI 構造の変更

既存の単一コマンド構造をサブコマンド構造に変更：
- `jj-desc` → `jj-desc generate` (既存機能、デフォルト動作として維持)
- `jj-desc backfill` → 新機能

## 受け入れ基準

1. `jj-desc backfill` で空 description のコミットに LLM 生成の description を設定できる
2. `--revisions` で対象範囲を柔軟に指定できる
3. `--dry-run` で実行前にプレビューできる
4. `--interactive` で各コミットごとに確認・スキップ・編集できる
5. エラーハンドリング（失敗したコミットをスキップして続行）
6. 既存の `jj-desc` コマンド（引数なし実行）が後方互換性を保つ

## 影響範囲

- CLI 構造（clap のサブコマンド化）
- jj コマンド実行ロジック（複数コミットの処理）
- エラーハンドリング（部分失敗の扱い）

## 非機能要件

- 大量コミット処理時の進捗表示
- API レート制限への配慮（遅延オプション）
- 処理中断時の安全性（途中で Ctrl+C しても問題ない）
