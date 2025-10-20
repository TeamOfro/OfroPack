use std::path::PathBuf;

use crate::cmd::Run;

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Model3D {
    /// カンマ区切りのマテリアルリスト (例: diamond_axe,iron_sword)
    #[arg(short, long, value_delimiter = ',', required = true)]
    materials: Vec<String>,

    /// カスタムモデルデータ名
    #[arg(short, long)]
    custom_model_data: String,

    /// モデルのJson
    mode_json_file: PathBuf,

    /// レイヤー画像ファイルのパス (複数指定可能)
    #[arg(required = true)]
    layer_images: Vec<PathBuf>,
}

impl Run for Model3D {
    fn run(&self) -> anyhow::Result<()> {
        println!(
            "Adding 3D model with custom_model_data '{}'",
            self.custom_model_data
        );
        // Here you would add the logic to handle adding the 3D model
        Ok(())
    }
}
