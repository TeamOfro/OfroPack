use anyhow::{Context, Result};

use crate::constants::Paths;
use crate::infra::FileSystem;
use crate::models::ModelFile;

/// Handles model JSON file generation
pub struct ModelGenerator;

impl ModelGenerator {
    /// Create a model JSON file for a custom model
    pub fn create(custom_model_data: &str) -> Result<()> {
        let model_path = Paths::model_path(custom_model_data);
        let model_file = ModelFile::new(custom_model_data);

        FileSystem::write_json(&model_path, &model_file).context(format!(
            "モデルファイルの作成に失敗: {}",
            model_path.display()
        ))?;

        println!("  ✓ モデル: {}", model_path.display());
        Ok(())
    }

    /// Check if model exists
    pub fn exists(custom_model_data: &str) -> bool {
        let model_path = Paths::model_path(custom_model_data);
        FileSystem::exists(&model_path)
    }

    /// Get model path
    pub fn path(custom_model_data: &str) -> std::path::PathBuf {
        Paths::model_path(custom_model_data)
    }
}
