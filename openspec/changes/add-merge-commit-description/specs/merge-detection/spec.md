# 仕様: マージコミット検出

## 概要
空の差分を持つマージコミットを検出し、適切なdescriptionを設定する機能。

## ADDED Requirements

### REQ-MERGE-001: マージコミット検出
`jj log`のテンプレート機能を使用してコミットの親の数を取得し、マージコミットかどうかを判定する。

#### Scenario: 通常コミットの判定
- **Given**: 親が1つのコミット
- **When**: `is_merge_commit`関数を呼び出す
- **Then**: `false`が返される

#### Scenario: マージコミットの判定
- **Given**: 親が2つ以上のコミット
- **When**: `is_merge_commit`関数を呼び出す
- **Then**: `true`が返される

#### Scenario: ルートコミットの判定
- **Given**: 親が0のコミット（ルート）
- **When**: `is_merge_commit`関数を呼び出す
- **Then**: `false`が返される

---

### REQ-MERGE-002: 空差分マージコミットへのdescription設定
差分が空でマージコミットの場合、「Merge commit」というdescriptionを自動設定する。

#### Scenario: マージコミットでdiffが空の場合
- **Given**: `jj diff`の出力が空
- **And**: コミットがマージコミット（親が2以上）
- **When**: `jj-desc`を実行
- **Then**: 「Merge commit」というdescriptionが設定される
- **And**: エラーは発生しない

#### Scenario: 非マージコミットでdiffが空の場合
- **Given**: `jj diff`の出力が空
- **And**: コミットが通常コミット（親が1）
- **When**: `jj-desc`を実行
- **Then**: `EmptyDiff`エラーが返される
- **And**: descriptionは変更されない

---

## MODIFIED Requirements

### REQ-DIFF-001: 空差分エラーの挙動変更
既存の空差分エラー処理を変更し、マージコミット判定を追加する。

#### Scenario: 空差分時のフォールバック（変更前）
- ~~**When**: diffが空の場合~~
- ~~**Then**: 即座に`EmptyDiff`エラーを返す~~

#### Scenario: 空差分時のフォールバック（変更後）
- **When**: diffが空の場合
- **Then**: マージコミットかどうかを判定する
- **And**: マージコミットなら「Merge commit」を設定
- **And**: 非マージコミットなら`EmptyDiff`エラーを返す

---

## 技術仕様

### 使用するjjコマンド
```bash
jj log -T 'parents.len()' -r <revision> --no-graph
```

### 出力形式
- 通常コミット: `1`
- マージコミット: `2`（または3以上）
- ルートコミット: `0`

### エラーハンドリング
- jjコマンドが失敗した場合: `JjDescError::JjCommand`エラー
- 出力のパースに失敗した場合: デフォルトで非マージ（false）として扱う
