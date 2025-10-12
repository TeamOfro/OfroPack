# GitHub Actions で Custom Model Data を追加する

このリポジトリでは、GitHub Actions を使って自動的に Custom Model Data を追加できます。

## 方法1: 手動でワークフローを実行

1. GitHubリポジトリの **Actions** タブに移動
2. 左サイドバーから **"Add Custom Model Data"** を選択
3. 右側の **"Run workflow"** をクリック
4. 必要な情報を入力：
   - **materials**: マテリアル名（カンマ区切り、例: `diamond_axe,iron_sword`）
   - **custom_model_data**: カスタムモデルデータ名（必須、小文字・数字・アンダースコアのみ）
   - **image_url**: 画像ファイルのURL（直接アクセス可能なURL）
5. **"Run workflow"** をクリック

ワークフローが完了すると、自動的にPull Requestが作成されます。

## 方法2: Issue から実行

1. GitHubリポジトリの **Issues** タブに移動
2. **"New issue"** をクリック
3. **"Request to add a custom model data"** テンプレートを選択
4. フォームに必要な情報を入力：
   - **Materials**: マテリアル名（カンマ区切り）
   - **Custom Model Data**: カスタムモデルデータ名（必須、小文字・数字・アンダースコアのみ）
   - **Image URL**: 画像ファイルのURL（直接アクセス可能なURL）
5. Issueを作成

Issueが作成されると、自動的にワークフローが実行され、Pull Requestが作成されます。
処理が完了すると、Issueにコメントが追加されます。

## 画像URLについて

画像URLは直接アクセス可能な公開URLである必要があります。以下のような方法で画像を公開できます：

- GitHub Issueやコメントに画像をドラッグ&ドロップして、生成されたURLを使用
- GitHub Releasesに画像をアップロード
- 外部の画像ホスティングサービス（Imgur、Discord添付など）

**注意**: ワークフローは画像が正しくダウンロードできたかを検証します。URLが無効だったり、PNG形式でない場合はエラーになります。

## 例

### 手動実行の例

```txt
materials: diamond_axe,iron_sword
custom_model_data: phantom_hunter
image_url: https://github.com/user/repo/assets/12345/phantom_hunter.png
```

### Issueの例

```txt
Materials: diamond_axe,iron_sword,golden_hoe
Custom Model Data: special_weapon
Image URL: https://i.imgur.com/example.png
```

## 注意事項

- マテリアル名は小文字で記述してください
- カスタムモデルデータ名は**必須**で、英小文字、数字、アンダースコアのみ使用可能
- 画像ファイルはPNG形式である必要があります
- ワークフローは画像のダウンロードとPNG形式を検証します
- 同じカスタムモデルデータ名が既に存在する場合、エラーになります
- 画像URLはクエリパラメータ付きでも正しく処理されます（Discord添付ファイルなど）
