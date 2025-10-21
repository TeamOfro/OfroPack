use std::path::PathBuf;

use anyhow::Context;

use crate::{
    cmd::Run,
    constants::{ItemModelParent, Paths, should_snake_case},
    pipeline::image_validator::ImageValidator,
    schema::{
        animation::{AnimationData, AnimationInfo},
        items::{ItemCase, ItemResource},
        models::ItemModel,
    },
    utils::json::{read_json, write_json},
};

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
        if self.materials.is_empty() {
            anyhow::bail!("少なくとも1つのmaterialを指定してください。");
        }

        self.materials
            .iter()
            .try_for_each(|material| should_snake_case(material))?;

        if !self.path_to_image.exists() {
            anyhow::bail!(
                "画像ファイルが存在しません: {}",
                self.path_to_image.to_string_lossy()
            );
        }

        let custom_model_data = match &self.custom_model_data {
            Some(name) => name.to_lowercase(),
            None => self
                .path_to_image
                .file_stem()
                .and_then(|s| s.to_str())
                .context("画像ファイル名の取得に失敗")?
                .to_string(),
        };

        should_snake_case(&custom_model_data)?;

        let model_path = Paths::model_path(&custom_model_data);
        let texture_path = Paths::texture_path(&custom_model_data);

        if model_path.exists() {
            anyhow::bail!(
                "モデルファイルが既に存在します: {}",
                model_path.to_string_lossy()
            );
        }

        if texture_path.exists() {
            anyhow::bail!(
                "テクスチャファイルが既に存在します: {}",
                texture_path.to_string_lossy()
            );
        }

        let animation_info = self.frametime.map(|frametime| AnimationInfo {
            animation: AnimationData { frametime },
        });

        ImageValidator::new_png(&self.path_to_image)?.should_model(animation_info.as_ref())?;

        let item_model = ItemModel::new(ItemModelParent::Handheld, &custom_model_data);
        write_json(&model_path, &item_model)
            .with_context(|| format!("モデルファイルの書き込みに失敗: {}", model_path.display()))?;

        std::fs::copy(&self.path_to_image, &texture_path).with_context(|| {
            format!(
                "テクスチャファイルのコピーに失敗: {} -> {}",
                self.path_to_image.display(),
                texture_path.display()
            )
        })?;

        let case = ItemCase::new(&custom_model_data);
        for material in &self.materials {
            let material_path = Paths::item_path(material);
            let mut resource = if material_path.exists() {
                read_json::<ItemResource>(&material_path).with_context(|| {
                    format!(
                        "マテリアルファイルの読み込みに失敗: {}",
                        material_path.display()
                    )
                })?
            } else {
                ItemResource::new(material)
            };

            resource.add_case(case.clone());

            write_json(&material_path, &resource).with_context(|| {
                format!(
                    "マテリアルファイルの書き込みに失敗: {}",
                    material_path.display()
                )
            })?;

            println!(
                "カスタムモデルデータ '{}' をマテリアル '{}' に追加しました",
                custom_model_data, material
            );
        }

        Ok(())
    }
}
