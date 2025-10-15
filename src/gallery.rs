use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::models::ItemOverride;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub materials: Vec<String>,
    pub texture_url: String,
    pub added_date: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

        let models_dir = Path::new("assets/minecraft/models/item");
        let textures_dir = Path::new("assets/minecraft/textures/item");
        let items_dir = Path::new("assets/minecraft/items");

        if !models_dir.exists() {
            println!("⚠️  警告: モデルディレクトリが見つかりません");
            let empty_data = GalleryData {
                models: vec![],
                count: 0,
            };
            self.write_gallery_data(&empty_data)?;
            return Ok(());
        }

        let mut models = Vec::new();

        // Scan all model files
        let model_files: Vec<_> = fs::read_dir(models_dir)
            .context("モデルディレクトリの読み込みに失敗")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .collect();

        println!("  ✓ {} 個のモデルファイルを検出", model_files.len());

        for entry in model_files {
            let model_path = entry.path();
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
            if !texture_path.exists() {
                eprintln!("  ⚠  警告: '{}' のテクスチャが見つかりません", model_name);
                continue;
            }

            // Find materials that use this model
            let materials = self.find_materials_for_model(&model_name, items_dir)?;

            // Get git metadata
            let (added_date, author) = self.get_git_metadata(&model_path)?;

            models.push(ModelInfo {
                name: model_name.clone(),
                materials,
                texture_url: format!("assets/minecraft/textures/item/{}.png", model_name),
                added_date,
                author,
            });
        }

        // Sort by added date (newest first)
        models.sort_by(|a, b| b.added_date.cmp(&a.added_date));

        let gallery_data = GalleryData {
            count: models.len(),
            models,
        };

        self.write_gallery_data(&gallery_data)?;

        println!(
            "\n✅ ギャラリーデータを生成しました: {} モデル",
            gallery_data.count
        );
        println!("  出力: {}", self.output_path.display());

        Ok(())
    }

    /// Find materials that use a specific model
    fn find_materials_for_model(&self, model_name: &str, items_dir: &Path) -> Result<Vec<String>> {
        let mut materials = Vec::new();

        if !items_dir.exists() {
            return Ok(materials);
        }

        let item_files = fs::read_dir(items_dir)
            .context("アイテムディレクトリの読み込みに失敗")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"));

        for entry in item_files {
            let item_path = entry.path();
            let material_name = item_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            // Read and parse item file
            if let Ok(content) = fs::read_to_string(&item_path)
                && let Ok(item_override) = serde_json::from_str::<ItemOverride>(&content)
            {
                // Check if this item uses the model
                if item_override
                    .model
                    .cases
                    .iter()
                    .any(|c| c.when == model_name)
                {
                    materials.push(material_name);
                }
            }
        }

        Ok(materials)
    }

    /// Get git metadata for a file
    fn get_git_metadata(&self, file_path: &Path) -> Result<(String, String)> {
        // Try to get git log for the file
        let output = Command::new("git")
            .args([
                "log",
                "--diff-filter=A",
                "--format=%aI|%an",
                "--",
                file_path.to_str().unwrap_or(""),
            ])
            .output();

        if let Ok(output) = output
            && output.status.success()
        {
            let log = String::from_utf8_lossy(&output.stdout);
            let log = log.trim();

            if !log.is_empty() {
                let parts: Vec<&str> = log.split('|').collect();
                if parts.len() >= 2 {
                    return Ok((parts[0].to_string(), parts[1].to_string()));
                }
            }
        }

        // Fallback to file modification time
        if let Ok(metadata) = fs::metadata(file_path)
            && let Ok(modified) = metadata.modified()
        {
            let datetime: chrono::DateTime<chrono::Utc> = modified.into();
            return Ok((datetime.to_rfc3339(), "Unknown".to_string()));
        }

        Ok((chrono::Utc::now().to_rfc3339(), "Unknown".to_string()))
    }

    /// Write gallery data to JSON file
    fn write_gallery_data(&self, data: &GalleryData) -> Result<()> {
        // Create parent directory if needed
        if let Some(parent) = self.output_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("ディレクトリの作成に失敗: {}", parent.display()))?;
        }

        let json = serde_json::to_string_pretty(data).context("JSONのシリアライズに失敗")?;

        fs::write(&self.output_path, json).context(format!(
            "ファイルの書き込みに失敗: {}",
            self.output_path.display()
        ))?;

        Ok(())
    }
}
