use anyhow::{Context, Result};

use crate::constants::Paths;
use crate::infra::FileSystem;
use crate::models::ItemOverride;

/// Manages item override JSON files
pub struct ItemManager;

impl ItemManager {
    /// Add custom model data reference to a material's item file
    pub fn add_material(material: &str, custom_model_data: &str) -> Result<()> {
        let item_path = Paths::item_path(material);

        let mut item_override = if FileSystem::exists(&item_path) {
            FileSystem::read_json(&item_path).context(format!(
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
            .any(|c| c.when == custom_model_data)
        {
            println!(
                "  ⚠  '{}': カスタムモデル '{}' は既に存在します（スキップ）",
                material, custom_model_data
            );
            return Ok(());
        }

        item_override.add_case(custom_model_data);
        FileSystem::write_json(&item_path, &item_override).context(format!(
            "アイテムJSONの書き込みに失敗: {}",
            item_path.display()
        ))?;

        println!("  ✓ '{}'", material);

        Ok(())
    }

    /// Find materials that use a specific custom model
    pub fn find_materials_for_model(custom_model_data: &str) -> Result<Vec<String>> {
        let mut materials = Vec::new();
        let items_dir = Paths::items_dir();

        if !FileSystem::exists(&items_dir) {
            return Ok(materials);
        }

        let item_files = FileSystem::list_files(&items_dir, Some("json"))?;

        for item_path in item_files {
            let material_name = item_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            // Read and parse item file
            if let Ok(item_override) = FileSystem::read_json::<ItemOverride>(&item_path) {
                // Check if this item uses the model
                if item_override
                    .model
                    .cases
                    .iter()
                    .any(|c| c.when == custom_model_data)
                {
                    materials.push(material_name);
                }
            }
        }

        Ok(materials)
    }
}
