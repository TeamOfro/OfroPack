use std::path::PathBuf;

use anyhow::Context;

use crate::{
    cmd::Run,
    pipeline::image_validator::ImageValidator,
    schema::animation::{AnimationData, AnimationInfo},
    types::ItemModelParent,
    utils::add as helpers,
};

/// 📄 2Dモデル（テクスチャ）を追加
///
/// PNG画像からアイテムモデルを作成し、指定したマテリアルに適用します。
#[derive(Debug, clap::Parser)]
#[command(
    about = "2Dモデル（テクスチャ）を追加",
    long_about = "PNG画像からアイテムモデルを作成し、指定したマテリアルに適用します。\n\n\
                  アニメーションテクスチャの場合は --frametime オプションで\n\
                  フレームごとのtick数を指定できます。"
)]
pub struct Model {
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
    /// 省略時は画像ファイル名から自動推定されます。
    /// スネークケース（小文字 + アンダースコア）で指定してください。
    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "カスタムモデルデータ名（省略時は画像ファイル名）"
    )]
    custom_model_data: Option<String>,

    /// アニメーションのフレームタイム（tick数）
    ///
    /// アニメーションテクスチャの場合のみ指定します。
    /// 1 tick = 1/20秒です。例: 5 tick = 0.25秒
    #[arg(
        short = 'f',
        long,
        value_name = "TICKS",
        help = "アニメーションフレームタイム（tick数、1tick=1/20秒）"
    )]
    frametime: Option<u32>,

    /// テクスチャ画像ファイルのパス
    ///
    /// PNG形式の画像を指定してください。
    /// 画像サイズは2の累乗（16x16, 32x32, 64x64など）である必要があります。
    #[arg(value_name = "IMAGE_FILE", help = "テクスチャ画像（PNG）のパス")]
    path_to_image: PathBuf,

    /// モデル親（2Dの描画スタイル）
    ///
    /// アイテムの表示スタイルを指定します。
    /// - handheld: 手持ちアイテム（デフォルト）
    /// - item_generated: 生成アイテム
    #[arg(
        long,
        value_enum,
        default_value = "handheld",
        value_name = "PARENT",
        help = "モデル親（表示スタイル）"
    )]
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
        println!("\n📄 2Dモデル追加を開始します...\n");

        helpers::validate_materials(&self.materials)?;

        if !self.path_to_image.exists() {
            anyhow::bail!(
                "❌ 画像ファイルが存在しません: {}",
                self.path_to_image.to_string_lossy()
            );
        }

        let custom_model_data =
            helpers::infer_or_validate_name(&self.custom_model_data, &self.path_to_image)?;

        println!("📋 カスタムモデルデータ名: {}", custom_model_data);
        println!("📦 適用マテリアル: {}", self.materials.join(", "));
        println!("🎨 モデル親: {}", self.parent.as_str());

        if let Some(ft) = self.frametime {
            println!("🎬 アニメーション: frametime = {} tick", ft);
        }
        println!();

        helpers::ensure_not_exists_2d(&custom_model_data)?;

        let animation_info = self.frametime.map(|frametime| AnimationInfo {
            animation: AnimationData { frametime },
        });

        println!("🔍 画像を検証中...");
        ImageValidator::new_png(&self.path_to_image)?.should_model(animation_info.as_ref())?;

        println!("📝 モデルファイルを作成中...");
        helpers::write_new_item_model(self.parent, &custom_model_data)?;

        if let Some(animation_info) = animation_info {
            println!("📝 アニメーションファイルを作成中...");
            helpers::write_new_animation(&custom_model_data, &animation_info)?;
        }

        println!("🖼️  テクスチャをコピー中...");
        let texture_path = crate::paths::Paths::texture_path(&custom_model_data);
        std::fs::copy(&self.path_to_image, &texture_path).with_context(|| {
            format!(
                "テクスチャファイルのコピーに失敗: {} -> {}",
                self.path_to_image.display(),
                texture_path.display()
            )
        })?;

        println!("⚙️  マテリアルに適用中...");
        helpers::update_materials(&self.materials, &custom_model_data)?;

        println!(
            "\n✅ 2Dモデル '{}' を正常に追加しました！\n",
            custom_model_data
        );

        Ok(())
    }
}
