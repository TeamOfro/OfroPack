use anyhow::Result;
use std::path::Path;

use crate::domain::{AnimationHandler, ModelGenerator, TextureHandler};
use crate::models::AnimationInfo;
use crate::services::ItemManager;

/// Orchestrates custom model processing operations
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
    ) -> Result<()> {
        // Validate that model doesn't already exist
        if ModelGenerator::exists(&self.custom_model_data) {
            anyhow::bail!(
                "カスタムモデルが既に存在します\n\
                場所: {}\n\n\
                💡 ヒント: 既存のモデルにマテリアルを追加する場合は 'extend' コマンドを使用してください:\n\
                processor extend -m <マテリアル名> -c {}",
                ModelGenerator::path(&self.custom_model_data).display(),
                self.custom_model_data
            );
        }

        // Install texture (validates and copies)
        TextureHandler::install(image_path, &self.custom_model_data, animation.as_ref())?;

        // Install animation if provided
        let animation = if let Some(anim) = animation {
            Some(AnimationHandler::install(
                image_path,
                &self.custom_model_data,
                &anim,
            )?)
        } else {
            None
        };

        // Process materials
        println!("📦 マテリアルを処理中...");
        for material in materials {
            ItemManager::add_material(material, &self.custom_model_data)?;
        }

        // Create model file
        ModelGenerator::create(&self.custom_model_data)?;

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
    pub fn extend_materials(&self, materials: &[String]) -> Result<()> {
        // Validate that model already exists
        if !ModelGenerator::exists(&self.custom_model_data) {
            anyhow::bail!(
                "カスタムモデルが見つかりません\n\
                場所: {}\n\n\
                💡 ヒント: 新しいモデルを作成する場合は 'add' コマンドを使用してください:\n\
                processor add -m <マテリアル名> -c {} <画像ファイル>",
                ModelGenerator::path(&self.custom_model_data).display(),
                self.custom_model_data
            );
        }

        if !TextureHandler::exists(&self.custom_model_data) {
            anyhow::bail!(
                "テクスチャファイルが見つかりません\n\
                場所: {}\n\n\
                カスタムモデルが不完全な状態です。",
                TextureHandler::path(&self.custom_model_data).display()
            );
        }

        // Process materials
        println!("📦 マテリアルを追加中...");
        for material in materials {
            ItemManager::add_material(material, &self.custom_model_data)?;
        }

        println!(
            "\n✅ カスタムモデル '{}' に {} マテリアルを追加しました",
            self.custom_model_data,
            materials.len()
        );

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
        let env = TestEnv::new();
        let processor = Processor::new("test_model".to_string());

        // Create test image
        let image_path = std::env::temp_dir().join("test_image.png");
        create_test_image(&image_path);

        let materials = vec!["diamond_sword".to_string()];
        let result = processor.add_with_texture(&materials, &image_path, None);

        assert!(result.is_ok());

        // Verify files were created
        assert!(ModelGenerator::exists("test_model"));
        assert!(TextureHandler::exists("test_model"));

        // Cleanup
        fs::remove_file(image_path).ok();
        drop(env);
    }

    #[test]
    fn test_add_with_texture_duplicate_model() {
        let env = TestEnv::new();
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
        drop(env);
    }

    #[test]
    fn test_extend_materials() {
        let env = TestEnv::new();
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

        // Cleanup
        fs::remove_file(image_path).ok();
        drop(env);
    }

    #[test]
    fn test_extend_materials_nonexistent_model() {
        let env = TestEnv::new();
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
        drop(env);
    }
}
