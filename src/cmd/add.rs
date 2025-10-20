use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Add {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum Subcommand {
    Model {
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
    },
    Model3D {
        /// カンマ区切りのマテリアルリスト (例: diamond_axe,iron_sword)
        #[arg(short, long, value_delimiter = ',', required = true)]
        materials: Vec<String>,

        /// カスタムモデルデータ名 (省略時はモデルファイル名を使用)
        #[arg(short, long)]
        custom_model_data: Option<String>,

        /// モデルのJson
        mode_json: String,

        /// レイヤー画像ファイルのパス (複数指定可能)
        #[arg(required = true)]
        layer_images: Vec<PathBuf>,
    },
}

impl super::Run for Add {
    fn run(&self) -> anyhow::Result<()> {
        // TODO:
        // まず処理する内容
        // - 普通のCustomModelの追加
        // - animation付きのCustomModelの追加
        // - blockbenchとかのelementsとか設定されるやつ (jsonの内容をinputで処理する)
        // これらの処理を実装する
        println!("Add command executed");
        Ok(())
    }
}
