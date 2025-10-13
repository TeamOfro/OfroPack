# OfroPack

Ofro鯖のMinecraftリソースパック

## 🚀 使い方

<https://teamofro.github.io/OfroPack/>

### ダウンロード

最新版は以下のURLからダウンロードできます：

```txt
https://teamofro.github.io/OfroPack/OfroPack.zip
```

### SHA1ハッシュ検証

```bash
curl -s https://teamofro.github.io/OfroPack/hash.txt
```

## ✨ カスタムモデル追加

Issueを作成するだけで自動的にリソースパックに追加されます。

1. [Issues](../../issues/new/choose)タブから **"Request to add a custom model data"** を選択
2. 必要な情報を入力：
   - **Materials**: `diamond_axe,iron_sword`
   - **Custom Model Data**: `my_model`（小文字・数字・アンダースコアのみ）
   - **Image URL**: 画像の公開URL
3. Issueを作成

数分後、自動的にPull Requestが作成され、IssueとPRの両方に**画像プレビュー（256x256）**が表示されます。
マージ後にGitHub Pagesに反映されます。

詳細は[GITHUB_ACTIONS.md](GITHUB_ACTIONS.md)を参照。

## 🏗️ 開発

### ローカルでの実行

```bash
# ツールのビルド
cargo build --release

# カスタムモデル追加
./target/release/processer -m diamond_axe -c my_model image.png

# リソースパック作成
zip -r OfroPack.zip assets/ pack.mcmeta
```

### プロジェクト構成

```txt
OfroPack/
├── assets/                 # リソースパックのアセット
│   └── minecraft/
│       ├── items/          # アイテムオーバーライド定義
│       ├── models/         # モデルファイル
│       └── textures/       # テクスチャ画像
├── pack.mcmeta            # リソースパックメタデータ
├── src/                   # Rustツール
│   ├── main.rs
│   ├── cli.rs
│   ├── constants.rs
│   ├── models.rs
│   ├── processor.rs
│   └── file_utils.rs
└── .github/
    ├── workflows/         # GitHub Actions
    └── ISSUE_TEMPLATE/    # Issueテンプレート
```

## 📦 自動化

- **Issue作成時**: カスタムモデルデータを自動追加してPR作成
- **main pushまたはPRマージ時**: GitHub Pagesに自動デプロイ

## 📝 ライセンス

[LICENSE](LICENSE)を参照
