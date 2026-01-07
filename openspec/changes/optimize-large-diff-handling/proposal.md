# 変更提案: 巨大diff最適化

## 概要

巨大なdiffがLLMに送信された際の問題（APIエラー、タイムアウト、高コスト）に対処するため、aicommit2を参考にdiff処理を最適化する。

## 背景

現在の`jj-desc`は`jj diff`の全出力をそのままLLMに送信している。これにより以下の問題が発生する可能性がある：

| 問題 | 影響 |
|------|------|
| コンテキストウィンドウ超過 | APIエラー (400/413) |
| 処理時間超過 | 30秒タイムアウト |
| 不要なトークン消費 | 高コスト（ロックファイルなど） |

## 参考: aicommit2の対応

[aicommit2](https://github.com/tak-bro/aicommit2)の実装を調査した結果、以下の対策が実装されている：

1. **ロックファイル自動除外**: `package-lock.json`, `*.lock` など
2. **バイナリファイル簡略化**: 詳細diffの代わりに `"Binary file X added"` のみ
3. **サイズ表示**: `Detected N staged files (M characters)` で警告
4. **CLIオプション**: `--exclude` でユーザー指定の除外

**注意**: aicommit2は閾値による強制制限は実装していない（READMEで「コミット単位を減らせ」と案内のみ）

## 提案内容

aicommit2に倣い、以下の機能を実装する：

### 1. ロックファイル自動除外
- `Cargo.lock`, `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`, `*.lock`, `*.lockb`

### 2. バイナリファイル簡略化
- バイナリファイルは `"Binary file {path} changed"` のみ送信

### 3. diffサイズ警告表示
- diffサイズを表示: `Diff size: 150KB (3500 lines)`
- 閾値（例: 50KB）超過時は警告表示
- **強制制限はしない**（aicommit2と同様）

### 4. `--exclude` オプション追加
- ユーザーが追加で除外するファイルパターンを指定可能

## スコープ外

- diffの自動切り詰め
- トークン数計算
- ファイル別分割処理

## 関連ドキュメント

- [aicommit2 git.ts](https://github.com/tak-bro/aicommit2/blob/main/src/utils/git.ts)
- [aicommit2 README](https://github.com/tak-bro/aicommit2)
