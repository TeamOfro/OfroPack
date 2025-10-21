use std::path::PathBuf;

use anyhow::Context;

use crate::{
    cmd::Run,
    constants::{Paths, should_snake_case},
    pipeline::image_validator::ImageValidator,
    schema::{
        items::{ItemCase, ItemResource},
        models::ItemModel,
    },
    utils::json::{merge_json, read_json, write_json},
};

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
    model_json_file: PathBuf,

    /// レイヤー画像ファイルのパス (複数指定可能)
    #[arg(required = true)]
    layer_images: Vec<PathBuf>,
}

impl Model3D {
    pub fn new(
        materials: Vec<String>,
        custom_model_data: String,
        model_json_file: PathBuf,
        layer_images: Vec<PathBuf>,
    ) -> Self {
        Self {
            materials,
            custom_model_data,
            model_json_file,
            layer_images,
        }
    }
}

impl Run for Model3D {
    fn run(&self) -> anyhow::Result<()> {
        if self.materials.is_empty() {
            anyhow::bail!("少なくとも1つのmaterialを指定してください。");
        }

        self.materials
            .iter()
            .try_for_each(|material| should_snake_case(material))?;

        if self.layer_images.is_empty() {
            anyhow::bail!("少なくとも1つのレイヤー画像ファイルを指定してください。");
        }

        should_snake_case(&self.custom_model_data)?;

        let model_path = Paths::model_path(&self.custom_model_data);

        if model_path.exists() {
            anyhow::bail!(
                "モデルファイルが既に存在します: {}",
                model_path.to_string_lossy()
            );
        }

        let mut model_value =
            read_json::<serde_json::Value>(&self.model_json_file).with_context(|| {
                format!(
                    "モデルJSONファイルの読み込みに失敗: {}",
                    self.model_json_file.to_string_lossy()
                )
            })?;

        let mut model =
            serde_json::from_value::<ItemModel>(model_value.clone()).with_context(|| {
                format!(
                    "モデルJSONのパースに失敗: {}",
                    self.model_json_file.to_string_lossy()
                )
            })?;

        let layer_count = model.textures.overwrite(&self.custom_model_data);

        if layer_count != self.layer_images.len() {
            anyhow::bail!(
                "モデルのレイヤー数({})と指定された画像ファイル数({})が一致しません。",
                layer_count,
                self.layer_images.len()
            );
        }

        let model = serde_json::to_value(&model).with_context(|| {
            format!(
                "モデルJSONのシリアライズに失敗: {}",
                self.model_json_file.to_string_lossy()
            )
        })?;

        model_value.get_mut("textures").map(|t| t.take());

        merge_json(&mut model_value, &model);

        write_json(&model_path, &model_value).with_context(|| {
            format!(
                "モデルファイルの書き込みに失敗: {}",
                model_path.to_string_lossy()
            )
        })?;

        for (i, layer_image) in self.layer_images.iter().enumerate() {
            if !layer_image.exists() {
                anyhow::bail!(
                    "レイヤー画像ファイルが存在しません: {}",
                    layer_image.to_string_lossy()
                );
            }

            ImageValidator::new_png(layer_image)?;

            let texture_path = Paths::texture_layer_path(&self.custom_model_data, i);

            std::fs::create_dir_all(
                texture_path
                    .parent()
                    .context("テクスチャパスの親ディレクトリの取得に失敗")?,
            )?;

            std::fs::copy(layer_image, &texture_path).with_context(|| {
                format!(
                    "レイヤー画像ファイルのコピーに失敗: {} -> {}",
                    layer_image.to_string_lossy(),
                    texture_path.to_string_lossy()
                )
            })?;
        }

        let case = ItemCase::new(&self.custom_model_data);
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
                self.custom_model_data, material
            );
        }

        Ok(())
    }
}
