# Spec: backfill-descriptions

## ADDED Requirements

#### Requirement: 複数コミットの一括 description 生成

`jj-desc backfill` コマンドで、指定した revset に含まれる空 description のコミットに対して、LLM を使用して description を自動生成・設定できる。

#### Scenario: デフォルトの mutable コミットに対する一括処理

**Given**: リポジトリに空 description のコミットが複数存在する  
**When**: `jj-desc backfill` を実行  
**Then**: 
- `mutable()` に含まれる空 description のコミットすべてに description が設定される
- 各コミットの処理結果（成功/失敗）が表示される
- 最終的に成功件数と失敗件数のサマリーが表示される

#### Scenario: revset で対象範囲を指定

**Given**: リポジトリに複数のブランチとコミットが存在する  
**When**: `jj-desc backfill --revisions "mine()"` を実行  
**Then**:
- 自分が作成したコミットのうち、空 description のものだけが処理される
- 他人のコミットや、既に description があるコミットは無視される

#### Scenario: dry-run モードでプレビュー

**Given**: 空 description のコミットが5件存在する  
**When**: `jj-desc backfill --dry-run` を実行  
**Then**:
- 各コミットに対して生成される description が表示される
- 実際には `jj describe` コマンドは実行されない
- "Generated description (not applied):" のような表示がある

#### Scenario: limit で処理件数を制限

**Given**: 空 description のコミットが20件存在する  
**When**: `jj-desc backfill --limit 5` を実行  
**Then**:
- 最初の5件だけが処理される
- 残りの15件は無視される
- サマリーに処理済み件数が表示される

#### Requirement: インタラクティブモード

`--interactive` オプションで、各コミットごとに生成された description を確認し、適用・スキップ・編集を選択できる。

#### Scenario: インタラクティブモードで個別確認

**Given**: 空 description のコミットが3件存在する  
**When**: `jj-desc backfill --interactive` を実行  
**Then**:
- 1件目のコミットの diff と生成された description が表示される
- ユーザーに選択肢が提示される: "Accept (a) / Skip (s) / Edit (e) / Quit (q)"
- 'a' を入力すると description が設定され、次のコミットに進む
- 's' を入力すると description を設定せず、次のコミットに進む
- 'e' を入力すると `$EDITOR` でテキストエディタが起動し、編集後に設定
- 'q' を入力すると処理を中断

#### Requirement: エラーハンドリング

個別コミットの処理に失敗した場合でも、他のコミットの処理を続行する。

#### Scenario: API エラー時の続行

**Given**: 空 description のコミットが5件存在する  
**When**: 3件目の処理中に API エラーが発生  
**Then**:
- 3件目のエラーメッセージが表示される
- 4件目、5件目の処理は続行される
- 最終サマリーに「成功: 4件、失敗: 1件」と表示される

#### Requirement: 後方互換性

既存の `jj-desc` コマンド（サブコマンドなし）の動作を維持する。

#### Scenario: 引数なし実行の互換性

**Given**: 既存のユーザーが `jj-desc` を引数なしで使用している  
**When**: `jj-desc` を実行（サブコマンドなし）  
**Then**:
- 現在のコミット (`@`) に対して description を生成する（既存の動作）
- エラーが発生しない

#### Scenario: 既存オプションの互換性

**Given**: 既存のオプション（`--revision`, `--dry-run` など）を使用  
**When**: `jj-desc --revision abc123` を実行  
**Then**:
- 指定したコミットに対して description を生成する
- `generate` サブコマンドと同じ動作をする

## MODIFIED Requirements

なし（既存仕様への変更なし、新機能の追加のみ）

## REMOVED Requirements

なし
