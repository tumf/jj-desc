# 提案: マージコミット検出とdescription設定

## 変更ID
`add-merge-commit-description`

## 概要
`jj diff`が空の場合でも、マージコミットであればその旨をdescriptionに設定する機能を追加する。

## 背景・動機
現在、`jj-desc`は`jj diff`の出力が空の場合にエラーを返している。しかし、jjではマージコミットは多くの場合「空」として扱われる（FAQより：「merge commits are often empty」）。マージコミットの場合は差分がなくても適切なdescriptionを設定できるようにしたい。

## 解決策
1. `jj log -T 'parents.len()' -r <rev> --no-graph`でコミットの親の数を取得
2. 親が2以上ならマージコミットと判断
3. マージコミットの場合は「Merge commit」などの適切なdescriptionを生成

## スコープ
- `src/jj.rs`: マージコミット検出関数の追加
- `src/main.rs`: 空diffの場合のフロー変更

## 関連ドキュメント
- [jj FAQ - Why are most merge commits marked as "empty"?](https://docs.jj-vcs.dev/latest/FAQ/#why-are-most-merge-commits-marked-as-empty)
