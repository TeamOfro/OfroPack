pub mod add;
pub mod extend;
mod generates;
mod metadata;
mod models;
mod runner;
mod zip;

/// 🎨 OfroPack - Minecraft リソースパック管理ツール
///
/// Minecraftのカスタムモデルデータを簡単に管理できるCLIツールです。
/// 2D/3Dモデルの追加、マテリアルの拡張、リソースパックの生成などを
/// コマンドラインから実行できます。
#[derive(Debug, clap::Parser)]
#[command(
    name = "processor",
    version,
    about = "🎨 OfroPack - Minecraft リソースパック管理ツール",
    long_about = "Minecraftのカスタムモデルデータを簡単に管理できるCLIツールです。\n\
                  2D/3Dモデルの追加、マテリアルの拡張、リソースパックの生成などを実行できます。",
    after_help = "詳細なドキュメント: https://github.com/TeamOfro/OfroPack"
)]
pub enum Cmd {
    /// ✨ カスタムモデルを追加
    ///
    /// 2D/3Dモデルをリソースパックに追加します。
    Add(add::Add),

    /// 🔧 既存モデルにマテリアルを追加
    ///
    /// 既存のカスタムモデルデータに新しいマテリアルを適用します。
    Extend(extend::Extend),

    /// 📊 ギャラリー用モデルデータを生成
    ///
    /// Webギャラリー表示用のmodels.jsonを生成します。
    Models(models::Models),

    /// 📝 メタデータを生成
    ///
    /// リソースパックのメタデータファイルを生成します。
    Metadata(metadata::Metadata),

    /// 📦 リソースパックをZip化
    ///
    /// assetsディレクトリとpack.mcmetaをZipファイルにまとめます。
    Zip(zip::Zip),

    /// 🤖 GitHub Actions用ランナー
    ///
    /// CI/CD環境でIssue処理などを実行します（通常は手動で使用しません）。
    #[command(hide = true)]
    Runner(runner::Runner),

    /// 🚀 すべてを生成（Zip + Models + Metadata）
    ///
    /// リソースパックのビルドに必要なすべてのファイルを一度に生成します。
    Generates(generates::Generates),
}

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

impl Run for Cmd {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Add(cmd) => cmd.run(),
            Self::Extend(cmd) => cmd.run(),
            Self::Models(cmd) => cmd.run(),
            Self::Metadata(cmd) => cmd.run(),
            Self::Zip(cmd) => cmd.run(),
            Self::Runner(cmd) => cmd.run(),
            Self::Generates(cmd) => cmd.run(),
        }
    }
}
