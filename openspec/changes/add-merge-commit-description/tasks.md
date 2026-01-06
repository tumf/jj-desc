# タスク: マージコミット検出とdescription設定

## タスク一覧

### 1. マージコミット検出関数の実装
- [x] `src/jj.rs`に`is_merge_commit`関数を追加
- [x] `jj log -T 'parents.len()' -r <rev> --no-graph`を実行
- [x] 親の数が2以上ならtrueを返す

### 2. メインフローの変更
- [x] `src/main.rs`でEmptyDiffエラー時にマージ判定を追加
- [x] マージコミットの場合は適切なdescriptionを設定
- [x] 非マージコミットの場合は従来通りエラーを返す

### 3. テストの追加
- [x] `is_merge_commit`関数の単体テスト
- [x] マージコミット時のE2Eテスト

### 4. ドキュメント更新
- [x] READMEにマージコミット対応について追記

## 依存関係
- タスク2はタスク1の完了が必要
- タスク3は タスク1,2の完了後に実施可能

## 検証方法
1. テストリポジトリでマージコミットを作成
   ```bash
   jj new main feature -m "test merge"
   ```
2. `jj-desc`を実行してdescriptionが設定されることを確認
