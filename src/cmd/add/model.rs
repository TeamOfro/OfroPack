use std::path::PathBuf;

use crate::cmd::Run;

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Model {
    /// カンマ区切りのマテリアルリスト (例: diamond_axe,iron_sword)
    #[arg(short, long, value_delimiter = ',', required = true)]
    materials: Vec<String>,

    /// カスタムモデルデータ名 (省略時は画像ファイル名を使用)
    #[arg(short, long)]
    custom_model_data: Option<String>,

    /// アニメーションのフレームタイム (tick数、アニメーションテクスチャの場合のみ指定)
    #[arg(short = 'f', long)]
    frametime: Option<u32>,

    /// テクスチャ画像ファイルのパス
    path_to_image: PathBuf,
}

impl Run for Model {
    fn run(&self) -> anyhow::Result<()> {
        println!(
            "Adding model with custom_model_data '{:?}'",
            self.custom_model_data
        );
        // Here you would add the logic to handle adding the model
        Ok(())
    }
}
