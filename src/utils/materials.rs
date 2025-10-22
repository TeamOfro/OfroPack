use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::paths::Paths;

#[derive(Debug, Deserialize)]
pub struct ItemTextureEntry {
    pub name: String,
    pub texture: String,
}

#[derive(Debug, Default, Clone)]
pub struct MaterialMapping {
    by_material: HashMap<String, String>,
}

impl MaterialMapping {
    pub fn load() -> anyhow::Result<Self> {
        let path = PathBuf::from(Paths::ITEMS_TEXTURES);
        let data = std::fs::read_to_string(&path)
            .with_context(|| format!("items_textures.json の読み込みに失敗: {}", path.display()))?;
        let entries: Vec<ItemTextureEntry> =
            serde_json::from_str(&data).context("items_textures.json のパースに失敗")?;
        let by_material = entries
            .into_iter()
            .filter_map(|e| {
                let texture = e.texture.trim_start_matches("minecraft:");
                let texture = match texture {
                    texture if texture.starts_with("item/") || texture.starts_with("items/") => {
                        format!("minecraft:item/{}", e.name)
                    }
                    texture if texture.starts_with("block/") || texture.starts_with("blocks/") => {
                        format!("minecraft:block/{}", e.name)
                    }
                    // skip other types
                    _ => {
                        return None;
                    }
                };

                Some((e.name, texture))
            })
            .collect();
        Ok(Self { by_material })
    }

    pub fn contains(&self, material: &str) -> bool {
        self.by_material.contains_key(material)
    }

    pub fn resolve_fallback_model_path(&self, material: &str) -> anyhow::Result<String> {
        self.by_material
            .get(material)
            .map(String::as_str)
            .map(String::from)
            .ok_or_else(|| {
                anyhow::anyhow!("マテリアル '{material}' のフォールバックモデルが見つかりません")
            })
    }
}
