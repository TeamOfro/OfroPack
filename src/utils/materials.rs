use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::constants::Paths;

#[derive(Debug, Deserialize)]
pub struct ItemTextureEntry {
    pub model: String,
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
            .map(|e| (e.model, e.texture))
            .collect::<HashMap<_, _>>();
        Ok(Self { by_material })
    }

    pub fn contains(&self, material: &str) -> bool {
        self.by_material.contains_key(material)
    }

    pub fn resolve_fallback_model_path(&self, material: &str) -> anyhow::Result<String> {
        let model = self.by_material.get(material).map(|s| s.as_str());
        match model {
            Some(m) => Ok(m.to_string()),
            None => anyhow::bail!(
                "マテリアル '{}' のフォールバックモデルが見つかりません",
                material
            ),
        }
    }
}
