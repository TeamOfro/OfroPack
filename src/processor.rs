use std::path::Path;

use anyhow::Context;

use crate::constants::Paths;
use crate::file_utils::{create_mcmeta_file, create_parent_dir_all, read_json, write_json};
use crate::image_validator::validate_image;
use crate::models::{AnimationInfo, ItemOverride, ModelFile};

pub struct Processor {
    pub custom_model_data: String,
}

impl Processor {
    pub fn new(custom_model_data: String) -> Self {
        Self { custom_model_data }
    }

    /// Add a new custom model with texture
    pub fn add_with_texture(
        &self,
        materials: &[String],
        image_path: &Path,
        animation: Option<AnimationInfo>,
    ) -> anyhow::Result<()> {
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
        let allow_animation = animation.is_some();
        let image_info = validate_image(image_path, allow_animation)?;
        println!("  ✓ 画像サイズ: {}", image_info.size_string());

        // Update animation with frame count if present
        let animation = animation.map(|anim| anim.with_frame_count(image_info.frame_count));

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

        // Create .mcmeta file if animation is provided
        if let Some(anim) = &animation {
            let mcmeta_path = create_mcmeta_file(&texture_path, anim.frametime)?;
            println!(
                "  ✓ アニメーション設定: {} (frametime: {})",
                mcmeta_path.display(),
                anim.frametime.get()
            );
        }

        let animation_note = if animation.is_some() {
            " (アニメーション)"
        } else {
            ""
        };

        println!(
            "\n✅ カスタムモデル '{}'{} を作成しました ({} マテリアル)",
            self.custom_model_data,
            animation_note,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    struct TestEnv {
        _temp_dir: TempDir,
    }

    impl TestEnv {
        fn new() -> Self {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path().to_str().unwrap().to_string();

            // Set up environment variable for test
            // Safety: Tests are single-threaded and this is the only place we set this variable
            unsafe {
                std::env::set_var("TEST_ROOT", &root);
            }

            // Create pack structure
            let assets_dir = temp_dir.path().join("assets").join("minecraft");
            fs::create_dir_all(assets_dir.join("items")).unwrap();
            fs::create_dir_all(assets_dir.join("models").join("item")).unwrap();
            fs::create_dir_all(assets_dir.join("textures").join("item")).unwrap();

            Self {
                _temp_dir: temp_dir,
            }
        }
    }

    fn create_test_image(path: &Path) {
        use image::{ImageBuffer, Rgba};
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(16, 16);
        img.save(path).unwrap();
    }

    #[test]
    fn test_add_with_texture_new_model() {
        let _env = TestEnv::new();
        let processor = Processor::new("test_model".to_string());

        // Create test image
        let image_path = std::env::temp_dir().join("test_image.png");
        create_test_image(&image_path);

        let materials = vec!["diamond_sword".to_string()];
        let result = processor.add_with_texture(&materials, &image_path, None);

        assert!(result.is_ok());

        // Verify files were created
        let model_path = Paths::model_path("test_model");
        let texture_path = Paths::texture_path("test_model");
        let item_path = Paths::item_path("diamond_sword");

        assert!(model_path.exists());
        assert!(texture_path.exists());
        assert!(item_path.exists());

        // Cleanup
        fs::remove_file(image_path).ok();
    }

    #[test]
    fn test_add_with_texture_duplicate_model() {
        let _env = TestEnv::new();
        let processor = Processor::new("duplicate_model".to_string());

        // Create test image
        let image_path = std::env::temp_dir().join("test_image_dup.png");
        create_test_image(&image_path);

        let materials = vec!["diamond_sword".to_string()];

        // First add should succeed
        processor
            .add_with_texture(&materials, &image_path, None)
            .unwrap();

        // Second add should fail
        let result = processor.add_with_texture(&materials, &image_path, None);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("カスタムモデルが既に存在します")
        );

        // Cleanup
        fs::remove_file(image_path).ok();
    }

    #[test]
    fn test_extend_materials() {
        let _env = TestEnv::new();
        let processor = Processor::new("extend_test".to_string());

        // Create test image and initial model
        let image_path = std::env::temp_dir().join("test_image_extend.png");
        create_test_image(&image_path);

        let initial_materials = vec!["diamond_sword".to_string()];
        processor
            .add_with_texture(&initial_materials, &image_path, None)
            .unwrap();

        // Extend with new material
        let new_materials = vec!["iron_sword".to_string()];
        let result = processor.extend_materials(&new_materials);

        assert!(result.is_ok());

        // Verify new item file was created
        let item_path = Paths::item_path("iron_sword");
        assert!(item_path.exists());

        // Cleanup
        fs::remove_file(image_path).ok();
    }

    #[test]
    fn test_extend_materials_nonexistent_model() {
        let _env = TestEnv::new();
        let processor = Processor::new("nonexistent_model".to_string());

        let materials = vec!["diamond_sword".to_string()];
        let result = processor.extend_materials(&materials);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("カスタムモデルが見つかりません")
        );
    }

    #[test]
    fn test_add_material_duplicate() {
        let _env = TestEnv::new();
        let processor = Processor::new("dup_material_test".to_string());

        // Create test image and model
        let image_path = std::env::temp_dir().join("test_image_dup_mat.png");
        create_test_image(&image_path);

        let materials = vec!["diamond_sword".to_string()];
        processor
            .add_with_texture(&materials, &image_path, None)
            .unwrap();

        // Try to extend with same material again (should skip duplicate)
        let result = processor.extend_materials(&materials);
        assert!(result.is_ok()); // Should succeed but skip

        // Verify only one entry exists
        let item_path = Paths::item_path("diamond_sword");
        let item_override: ItemOverride = read_json(&item_path).unwrap();
        assert_eq!(
            item_override
                .model
                .cases
                .iter()
                .filter(|c| c.when == "dup_material_test")
                .count(),
            1
        );

        // Cleanup
        fs::remove_file(image_path).ok();
    }
}
