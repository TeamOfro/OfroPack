pub mod model;
pub mod model3d;

/// ✨ カスタムモデル追加コマンド
#[derive(clap::Parser, Debug)]
#[command(
    about = "カスタムモデルをリソースパックに追加します",
    long_about = "2Dモデル（テクスチャ）または3Dモデル（JSON + テクスチャレイヤー）を\n\
                  リソースパックに追加します。"
)]
pub struct Add {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

/// サブコマンド
#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum Subcommand {
    /// 📄 2Dモデル（テクスチャ）を追加
    ///
    /// PNG画像からアイテムモデルを作成します。
    /// 通常の静止画またはアニメーションテクスチャに対応しています。
    #[command(visible_alias = "2d")]
    Model(model::Model),

    /// 🎲 3Dモデル（JSON + レイヤー）を追加
    ///
    /// モデルJSONファイルと複数のテクスチャレイヤーから3Dモデルを作成します。
    #[command(visible_alias = "3d")]
    Model3D(model3d::Model3D),
}

impl super::Run for Subcommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Model(cmd) => cmd.run(),
            Self::Model3D(cmd) => cmd.run(),
        }
    }
}

impl super::Run for Add {
    fn run(&self) -> anyhow::Result<()> {
        self.subcommand.run()
    }
}
