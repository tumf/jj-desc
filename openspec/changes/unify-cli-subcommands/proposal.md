# 変更提案: unify-cli-subcommands

## 概要

`generate` と `backfill` サブコマンドを統合し、サブコマンドなしのシンプルなCLIに変更する。

## 動機

現在の CLI には2つのサブコマンドがある:
- `generate`: 単一コミットの説明を生成（`--revision` オプション）
- `backfill`: 複数コミットの説明を生成（`--revisions` オプション）

これらは本質的に同じ機能であり、対象リビジョンの指定方法が異なるだけ。
統合することで:
- ユーザーの認知負荷を軽減
- `jj describe` に似た直感的なインターフェース
- コードの重複を削減

## 変更後の使用例

```bash
jj-desc                           # デフォルト: ::@ & mutable() の空説明コミット
jj-desc -r @                      # 単一コミット（現在のワーキングコピー）
jj-desc -r "mutable()"            # 複数コミット
jj-desc -r @ --dry-run            # プレビューモード
jj-desc -r "::@" -n 5             # 最大5件まで処理
jj-desc -r @ --interactive        # 対話モード
```

## 影響範囲

- `src/cli.rs`: サブコマンド定義の削除、オプションの統合
- `src/main.rs`: コマンド分岐ロジックの簡素化
- `src/commands/`: `generate.rs` と `backfill.rs` の統合検討
- ドキュメント: README、ヘルプメッセージの更新

## 後方互換性

- サブコマンドなしの実行は既存の `backfill` デフォルト動作を維持
- `--revision` と `--revisions` は `-r/--revisions` に統一（単一でも複数でも同じオプション）

## 関連仕様

- `openspec/specs/backfill-descriptions/spec.md` の更新が必要
