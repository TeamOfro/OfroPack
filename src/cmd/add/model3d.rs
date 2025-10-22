use std::path::PathBuf;

use anyhow::Context;

use crate::{
    cmd::Run,
    paths::Paths,
    pipeline::image_validator::ImageValidator,
    schema::models::ItemModel,
    utils::add as helpers,
    utils::json::{merge_json, read_json, write_json},
    validation::should_snake_case,
};

/// 🎲 3Dモデル（JSON + レイヤー）を追加
///
/// モデルJSONファイルと複数のテクスチャレイヤーから3Dモデルを作成し、
/// 指定したマテリアルに適用します。
#[derive(Debug, clap::Parser)]
#[command(
    about = "3Dモデルを追加",
    long_about = "モデルJSONファイルと複数のテクスチャレイヤーから3Dモデルを作成します。\n\n\
                  モデルJSON内のテクスチャパスは自動的に更新されます。"
)]
pub struct Model3D {
    /// カンマ区切りのマテリアルリスト
    ///
    /// 例: diamond_axe,iron_sword,golden_pickaxe
    #[arg(
        short,
        long,
        value_delimiter = ',',
        required = true,
        value_name = "MATERIALS",
        help = "適用するマテリアル（カンマ区切り）"
    )]
    materials: Vec<String>,

    /// カスタムモデルデータ名
    ///
    /// スネークケース（小文字 + アンダースコア）で指定してください。
    #[arg(short, long, value_name = "NAME", help = "カスタムモデルデータ名")]
    custom_model_data: String,

    /// モデルのJSONファイル
    ///
    /// Minecraft形式のモデルJSONファイルを指定します。
    #[arg(value_name = "MODEL_JSON", help = "モデルJSONファイルのパス")]
    model_json_file: PathBuf,

    /// レイヤー画像ファイルのパス（複数指定可能）
    ///
    /// モデルで使用するテクスチャレイヤーをすべて指定します。
    /// PNG形式で、モデルJSONで定義されたレイヤー数と一致する必要があります。
    #[arg(
        required = true,
        value_name = "LAYER_IMAGES",
        help = "テクスチャレイヤー画像（PNG）のパス"
    )]
    layer_images: Vec<PathBuf>,
}

impl Model3D {
    #[must_use]
    pub const fn new(
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
        helpers::validate_materials(&self.materials)?;

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

        model_value.get_mut("textures").map(serde_json::Value::take);
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

        helpers::update_materials(&self.materials, &self.custom_model_data)?;

        Ok(())
    }
}
