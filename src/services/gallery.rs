use anyhow::Result;
use std::path::PathBuf;

use crate::constants::Paths;
use crate::domain::AnimationHandler;
use crate::infra::{FileSystem, GitMetadata};
use crate::services::ItemManager;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub materials: Vec<String>,
    pub texture_url: String,
    pub added_date: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<crate::domain::AnimationMetadata>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GalleryData {
    pub models: Vec<ModelInfo>,
    pub count: usize,
}

pub struct GalleryGenerator {
    output_path: PathBuf,
}

impl GalleryGenerator {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path }
    }

    /// Generate models.json for gallery
    pub fn generate(&self) -> Result<()> {
        println!("🔍 カスタムモデル情報を収集中...");

        let models_dir = Paths::models_dir();
        let textures_dir = Paths::textures_dir();

        if !FileSystem::exists(&models_dir) {
            println!("⚠️  警告: モデルディレクトリが見つかりません");
            let empty_data = GalleryData {
                models: vec![],
                count: 0,
            };
            FileSystem::write_json(&self.output_path, &empty_data)?;
            return Ok(());
        }

        let mut models = Vec::new();

        // Scan all model files
        let model_files = FileSystem::list_files(&models_dir, Some("json"))?;
        println!("  ✓ {} 個のモデルファイルを検出", model_files.len());

        for model_path in model_files {
            let model_name = model_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            // Skip minecraft namespace models
            if model_name.contains("minecraft:") {
                continue;
            }

            // Check if texture exists
            let texture_path = textures_dir.join(format!("{}.png", model_name));
            if !FileSystem::exists(&texture_path) {
                eprintln!("  ⚠  警告: '{}' のテクスチャが見つかりません", model_name);
                continue;
            }

            // Find materials that use this model
            let materials = ItemManager::find_materials_for_model(&model_name)?;

            // Get git metadata
            let git_metadata =
                GitMetadata::for_file(&model_path).unwrap_or_else(|_| GitMetadata::unknown());

            // Check for animation metadata (.mcmeta file)
            let animation = AnimationHandler::get_metadata(&model_name, &textures_dir)?;

            models.push(ModelInfo {
                name: model_name.clone(),
                materials,
                texture_url: format!("assets/minecraft/textures/item/{}.png", model_name),
                added_date: git_metadata.added_date,
                author: git_metadata.author,
                animation,
            });
        }

        // Sort by added date (newest first)
        models.sort_by(|a, b| b.added_date.cmp(&a.added_date));

        let gallery_data = GalleryData {
            count: models.len(),
            models,
        };

        FileSystem::write_json(&self.output_path, &gallery_data)?;

        println!(
            "\n✅ ギャラリーデータを生成しました: {} モデル",
            gallery_data.count
        );
        println!("  出力: {}", self.output_path.display());

        Ok(())
    }
}
