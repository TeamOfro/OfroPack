use std::path::Path;

use anyhow::Context;

use crate::constants::Paths;
use crate::file_utils::{create_parent_dir_all, read_json, write_json};
use crate::image_validator::validate_image;
use crate::models::{ItemOverride, ModelFile};

pub struct Processor {
    pub custom_model_data: String,
}

impl Processor {
    pub fn new(custom_model_data: String) -> Self {
        Self { custom_model_data }
    }

    /// Add a new custom model with texture
    pub fn add_with_texture(&self, materials: &[String], image_path: &Path) -> anyhow::Result<()> {
        let model_path = Paths::model_path(&self.custom_model_data);
        let texture_path = Paths::texture_path(&self.custom_model_data);

        // Validate that model doesn't already exist
        if model_path.exists() {
            anyhow::bail!(
                "カスタムモデルが既に存在します\n\
                場所: {}\n\n\
                💡 ヒント: 既存のモデルにマテリアルを追加する場合は 'extend' コマンドを使用してください:\n\
                processor extend -m <マテリアル名> -c {}",
                model_path.display(),
                self.custom_model_data
            );
        }

        // Validate texture doesn't already exist
        if texture_path.exists() {
            eprintln!(
                "⚠️  警告: テクスチャファイルが既に存在します: {}\n\
                上書きされます。",
                texture_path.display()
            );
        }

        // Validate image
        println!("🔍 画像を検証中...");
        let image_info = validate_image(image_path)?;
        println!("  ✓ 画像サイズ: {}", image_info.size_string());

        // Prepare directories
        create_parent_dir_all(&model_path)?;
        create_parent_dir_all(&texture_path)?;

        // Process materials
        println!("📦 マテリアルを処理中...");
        for material in materials {
            self.add_material_to_item(material)?;
        }

        // Create model file
        self.create_model_file(&model_path)?;

        // Copy texture
        std::fs::copy(image_path, &texture_path).context(format!(
            "テクスチャをコピーできませんでした: {}",
            texture_path.display()
        ))?;
        println!("  ✓ テクスチャ: {}", texture_path.display());

        println!(
            "\n✅ カスタムモデル '{}' を作成しました ({} マテリアル)",
            self.custom_model_data,
            materials.len()
        );

        Ok(())
    }

    /// Add materials to an existing custom model
    pub fn extend_materials(&self, materials: &[String]) -> anyhow::Result<()> {
        let model_path = Paths::model_path(&self.custom_model_data);
        let texture_path = Paths::texture_path(&self.custom_model_data);

        // Validate that model already exists
        if !model_path.exists() {
            anyhow::bail!(
                "カスタムモデルが見つかりません\n\
                場所: {}\n\n\
                💡 ヒント: 新しいモデルを作成する場合は 'add' コマンドを使用してください:\n\
                processor add -m <マテリアル名> -c {} <画像ファイル>",
                model_path.display(),
                self.custom_model_data
            );
        }

        if !texture_path.exists() {
            anyhow::bail!(
                "テクスチャファイルが見つかりません\n\
                場所: {}\n\n\
                カスタムモデルが不完全な状態です。",
                texture_path.display()
            );
        }

        // Process materials
        println!("📦 マテリアルを追加中...");
        for material in materials {
            self.add_material_to_item(material)?;
        }

        println!(
            "\n✅ カスタムモデル '{}' に {} マテリアルを追加しました",
            self.custom_model_data,
            materials.len()
        );

        Ok(())
    }

    /// Add custom model data reference to a material's item file
    fn add_material_to_item(&self, material: &str) -> anyhow::Result<()> {
        let item_path = Paths::item_path(material);
        create_parent_dir_all(&item_path)?;

        let mut item_override = if item_path.exists() {
            read_json(&item_path).context(format!(
                "アイテムJSONの読み込みに失敗: {}",
                item_path.display()
            ))?
        } else {
            ItemOverride::new(material)
        };

        // Check if this custom model data is already added to avoid duplicates
        if item_override
            .model
            .cases
            .iter()
            .any(|c| c.when == self.custom_model_data)
        {
            println!(
                "  ⚠  '{}': カスタムモデル '{}' は既に存在します（スキップ）",
                material, self.custom_model_data
            );
            return Ok(());
        }

        item_override.add_case(&self.custom_model_data);
        write_json(&item_path, &item_override).context(format!(
            "アイテムJSONの書き込みに失敗: {}",
            item_path.display()
        ))?;

        println!("  ✓ '{}'", material);

        Ok(())
    }

    /// Create the model JSON file
    fn create_model_file(&self, model_path: &Path) -> anyhow::Result<()> {
        let model_file = ModelFile::new(&self.custom_model_data);
        write_json(model_path, &model_file).context(format!(
            "モデルファイルの作成に失敗: {}",
            model_path.display()
        ))?;
        println!("  ✓ モデル: {}", model_path.display());
        Ok(())
    }
}
