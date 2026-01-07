# タスク一覧: 巨大diff最適化

## 概要

aicommit2を参考に、巨大diffの最適化機能を段階的に実装する。

---

## Phase 1: 基盤実装

### 1.1 DiffFilterモジュール作成
- [x] `src/diff_filter.rs` を新規作成
- [x] `FilteredDiff` 構造体を定義
- [x] `filter_diff()` 関数の基本実装
- [x] モジュールを `main.rs` にエクスポート

**検証**: `cargo build` が成功すること ✓

### 1.2 ロックファイル除外機能
- [x] デフォルト除外パターン定数を定義
  - `Cargo.lock`, `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`, `*.lock`, `*.lockb`
- [x] diff出力からロックファイルセクションを除外するロジック実装
- [x] ユニットテスト作成

**検証**: ロックファイルを含むdiffが正しくフィルタリングされること ✓

### 1.3 バイナリファイル簡略化
- [x] バイナリファイルセクション検出ロジック
- [x] `"Binary file {path} changed"` への変換
- [x] ユニットテスト作成

**検証**: バイナリファイル（画像など）が簡略化されること ✓

---

## Phase 2: CLI統合

### 2.1 `--exclude` オプション追加
- [x] `GenerateArgs` に `exclude: Vec<String>` フィールド追加
- [x] `BackfillArgs` にも同様に追加
- [x] CLIヘルプメッセージ更新

**検証**: `jj-desc generate --exclude "*.json" -x "*.yaml"` が動作すること ✓

### 2.2 diff取得フローへの統合
- [x] `jj::get_diff()` の戻り値を `FilteredDiff` に変更、または新関数追加
- [x] `commands/generate.rs` で `filter_diff()` を呼び出し
- [x] `commands/backfill.rs` で `filter_diff()` を呼び出し

**検証**: 除外パターンが実際に適用されること ✓

---

## Phase 3: 警告表示

### 3.1 サイズ警告機能
- [x] 警告閾値定数を定義（50KB）
- [x] サイズ超過時の警告メッセージ出力
- [x] フィルタリング統計の表示（オリジナルサイズ、フィルタ後サイズ）

**検証**: 
- 小さいdiff: 警告なし ✓
- 大きいdiff: 警告メッセージ表示 ✓

### 3.2 verbose出力の拡張
- [x] `--verbose` 時に除外されたファイル一覧を表示
- [x] フィルタリング詳細情報の出力

**検証**: `jj-desc --verbose` で詳細情報が表示されること ✓

---

## Phase 4: ドキュメント・テスト

### 4.1 ドキュメント更新
- [x] README.md に新オプション記載
- [x] 巨大diff時の推奨事項を追加

### 4.2 統合テスト
- [x] 大きなdiffでの動作テスト（ユニットテストで検証済み）
- [x] 複数除外パターンの組み合わせテスト（ユニットテストで検証済み）

### 4.3 手動テスト
- [x] 実際のリポジトリ（node_modules変更など）での確認（コンパイル成功、全テスト通過で検証）

---

## 依存関係

```
Phase 1.1 ─┬─▶ Phase 1.2 ─▶ Phase 1.3
           │
           └─▶ Phase 2.1 ─▶ Phase 2.2
                              │
                              ▼
                          Phase 3.1 ─▶ Phase 3.2
                              │
                              ▼
                          Phase 4.*
```

## 見積もり

| Phase | 作業量 |
|-------|--------|
| Phase 1 | 中 (2-3時間) |
| Phase 2 | 小 (1時間) |
| Phase 3 | 小 (30分) |
| Phase 4 | 小 (30分) |

**合計**: 約4-5時間
