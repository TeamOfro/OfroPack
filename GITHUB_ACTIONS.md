# GitHub Actions で Custom Model Data を追加する

このリポジトリでは、GitHub Actions を使って自動的に Custom Model Data を追加できます。

## 方法: Issue から実行

1. GitHubリポジトリの **Issues** タブに移動
2. **"New issue"** をクリック
3. **"Request to add a custom model data"** テンプレートを選択
4. フォームに必要な情報を入力：
   - **Materials**: マテリアル名（カンマ区切り）
   - **Custom Model Data**: カスタムモデルデータ名（必須、小文字・数字・アンダースコアのみ）
   - **Image URL**: 画像ファイルのURL（直接アクセス可能なURL）
5. Issueを作成

Issueが作成されると、自動的にワークフローが実行され、Pull Requestが作成されます。
処理が完了すると、Issueにコメントが追加され、**画像のプレビュー（256x256）**が表示されます。

**重要**: エラーが発生した場合、Issueは自動的にクローズされます。修正する場合は、**新しいIssueを作成**してください（既存のIssueを編集しても再実行されません）。

## 画像URLについて

画像URLは直接アクセス可能な公開URLである必要があります。以下のような方法で画像を公開できます：

- 外部の画像ホスティングサービス（Imgur、imgbb、Gyazoなど）
- Discord添付ファイルのURL（クエリパラメータ付きでも正しく処理されます）

**注意**: ワークフローは画像が正しくダウンロードできたかを検証します。URLが無効だったり、PNG形式でない場合はエラーになります。

## 例

```txt
Materials: diamond_axe,iron_sword,golden_hoe
Custom Model Data: special_weapon
Image URL: https://i.imgur.com/example.png
```

## 注意事項

- マテリアル名は小文字で記述してください
- カスタムモデルデータ名は**必須**で、英小文字、数字、アンダースコアのみ使用可能
- 画像ファイルはPNG形式である必要があります
- 画像URLは直接アクセス可能な公開URLを使用してください（Imgur、Discord添付など）
- ワークフローは画像のダウンロードとPNG形式を検証します
- 同じカスタムモデルデータ名が既に存在する場合、エラーになります
- **エラーが発生した場合、Issueは自動的にクローズされます。新しいIssueを作成してください**
- Issueの編集では再実行されません（意図的な仕様）
