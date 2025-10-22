use std::path::PathBuf;

use anyhow::Context;

use crate::{
    cmd::Run,
    pipeline::image_validator::ImageValidator,
    schema::animation::{AnimationData, AnimationInfo},
    types::ItemModelParent,
    utils::add as helpers,
};

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Model {
    /// カンマ区切りのマテリアルリスト (例: `diamond_axe,iron_sword`)
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

    /// モデル親（2Dの描画スタイル）
    #[arg(long, value_enum, default_value = "handheld")]
    parent: ItemModelParent,
}

impl Model {
    #[must_use]
    pub const fn new(
        materials: Vec<String>,
        custom_model_data: Option<String>,
        frametime: Option<u32>,
        path_to_image: PathBuf,
        parent: ItemModelParent,
    ) -> Self {
        Self {
            materials,
            custom_model_data,
            frametime,
            path_to_image,
            parent,
        }
    }
}

impl Run for Model {
    fn run(&self) -> anyhow::Result<()> {
        helpers::validate_materials(&self.materials)?;

        if !self.path_to_image.exists() {
            anyhow::bail!(
                "画像ファイルが存在しません: {}",
                self.path_to_image.to_string_lossy()
            );
        }

        let custom_model_data =
            helpers::infer_or_validate_name(&self.custom_model_data, &self.path_to_image)?;

        helpers::ensure_not_exists_2d(&custom_model_data)?;

        let animation_info = self.frametime.map(|frametime| AnimationInfo {
            animation: AnimationData { frametime },
        });

        ImageValidator::new_png(&self.path_to_image)?.should_model(animation_info.as_ref())?;

        helpers::write_new_item_model(self.parent, &custom_model_data)?;

        let texture_path = crate::paths::Paths::texture_path(&custom_model_data);
        std::fs::copy(&self.path_to_image, &texture_path).with_context(|| {
            format!(
                "テクスチャファイルのコピーに失敗: {} -> {}",
                self.path_to_image.display(),
                texture_path.display()
            )
        })?;

        helpers::update_materials(&self.materials, &custom_model_data)?;

        Ok(())
    }
}
