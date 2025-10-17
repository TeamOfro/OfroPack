# 🎨 OfroPack

[![GitHub Pages](https://img.shields.io/badge/GitHub%20Pages-Live-success?style=flat&logo=github)](https://teamofro.github.io/OfroPack/)
[![License](https://img.shields.io/github/license/TeamOfro/OfroPack)](LICENSE)
[![Latest PR](https://img.shields.io/github/issues-pr-closed/TeamOfro/OfroPack?label=PRs)](https://github.com/TeamOfro/OfroPack/pulls?q=is%3Apr+is%3Aclosed)

Ofro鯖のMinecraftリソースパック - Issueから簡単にカスタムモデルを追加できる自動化システム

## ✨ 特徴

- 🤖 **完全自動化**: Issueを作成するだけで自動的にカスタムモデルが追加される
- 🎨 **ギャラリー**: 全てのカスタムモデルをWebで閲覧可能
- 🔍 **フィルタ機能**: マテリアル、作者、IDで検索・フィルタ
- 🚀 **高速処理**: Rust製CLIツールによる高速なビルド
- 📦 **自動デプロイ**: GitHub Pagesへの自動デプロイ

## 🚀 使い方

**Webサイト**: <https://teamofro.github.io/OfroPack/>

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

### 手順

1. **[Issues](../../issues/new/choose)** タブから **"カスタムモデルデータの追加リクエスト"** を選択
2. 必要な情報を入力：
   - **マテリアル**: `diamond_axe,iron_sword`（カンマ区切り）
   - **カスタムモデルデータ名**: `my_model`（小文字・数字・アンダースコアのみ）
   - **画像URL**: 画像の公開URL（Imgur、Discord添付など）
3. Issueを作成

### 処理の流れ

```txt
Issue作成 → 画像検証 → モデル追加 → PR自動作成 → レビュー → マージ → デプロイ
```

数分後、自動的にPull Requestが作成され、IssueとPRの両方に**画像プレビュー（256×256）**が表示されます。
マージ後、自動的にGitHub Pagesに反映されます。

**注意事項:**

- エラーが発生した場合、Issueは自動的にクローズされます
- 画像はPNG形式である必要があります
- 同じIDが既に存在する場合はエラーになります

詳細は[GITHUB_ACTIONS.md](GITHUB_ACTIONS.md)を参照。

## 🏗️ 開発

### ローカルでの実行

```bash
# ツールのビルド
cargo build --release

# カスタムモデル追加
./target/release/processor -m diamond_axe -c my_model image.png

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

このプロジェクトは完全に自動化されています：

- **Issue作成時**:
  - 画像のダウンロードと検証
  - カスタムモデルデータの追加
  - プレビュー画像の生成（256×256）
  - Pull Requestの自動作成
  - Issueへのフィードバックコメント

- **main pushまたはPRマージ時**:
  - リソースパックの自動ビルド（zip形式）
  - SHA1ハッシュの計算
  - ギャラリーデータ（models.json）の生成
  - GitHub Pagesへの自動デプロイ
  - メタデータファイルの生成

すべての処理はRust製CLIツールによって高速に実行されます。

## 📝 ライセンス

[LICENSE](LICENSE)を参照

## 🖥️ ローカル開発

### ギャラリーのローカルプレビュー

```bash
# 1. ギャラリーデータを生成
cargo run --release -- generate-gallery

# 2. ローカルサーバーを起動
python3 -m http.server 8000

# 3. ブラウザで開く
# http://localhost:8000/gallery.html
```

または、Rustの簡易HTTPサーバーを使用:

```bash
cargo install miniserve
miniserve . -p 8000
```

### CLIコマンド

```bash
# 新しいカスタムモデルを追加
./target/release/processor add -m diamond_axe,iron_sword -c my_model image.png

# 既存モデルにマテリアルを追加
./target/release/processor extend -m golden_hoe -c my_model

# ギャラリーデータを生成
./target/release/processor generate-gallery -o models.json
```

## 🤝 コントリビューション

コントリビューションを歓迎します！

1. このリポジトリをフォーク
2. 新しいブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. Pull Requestを作成

## 🐛 トラブルシューティング

### Issue作成後にエラーが発生する

- 画像URLが直接アクセス可能か確認してください
- PNG形式の画像を使用してください
- カスタムモデルデータ名が既に存在しないか確認してください

### ローカルビルドが失敗する

```bash
# 依存関係を再インストール
cargo clean
cargo build --release

# Rustのバージョンを確認（最新の安定版を推奨）
rustc --version
```
