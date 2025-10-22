//! マテリアルマッピング管理
//!
//! `items_textures.json` からマテリアルとテクスチャのマッピングを
//! 読み込み、管理します。

use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::paths::Paths;

/// アイテムテクスチャエントリ
#[derive(Debug, Deserialize)]
pub struct ItemTextureEntry {
    /// アイテム名
    pub name: String,
    /// テクスチャパス
    pub texture: String,
}

/// マテリアルマッピング
///
/// マテリアル名からフォールバックモデルパスへのマッピングを管理します。
#[derive(Debug, Default, Clone)]
pub struct MaterialMapping {
    by_material: HashMap<String, String>,
}

impl MaterialMapping {
    /// `items_textures.json` からマッピングを読み込む
    ///
    /// # Errors
    ///
    /// ファイルの読み込みまたはパースに失敗した場合
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

    /// マテリアルが存在するか確認
    pub fn contains(&self, material: &str) -> bool {
        self.by_material.contains_key(material)
    }

    /// マテリアルのフォールバックモデルパスを取得
    ///
    /// # Errors
    ///
    /// マテリアルが見つからない場合
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_mapping_contains() {
        let mut mapping = MaterialMapping::default();
        mapping.by_material.insert(
            "diamond_sword".to_string(),
            "minecraft:item/diamond_sword".to_string(),
        );

        assert!(mapping.contains("diamond_sword"));
        assert!(!mapping.contains("iron_sword"));
    }

    #[test]
    fn test_material_mapping_resolve() {
        let mut mapping = MaterialMapping::default();
        mapping.by_material.insert(
            "diamond_sword".to_string(),
            "minecraft:item/diamond_sword".to_string(),
        );

        let result = mapping.resolve_fallback_model_path("diamond_sword");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "minecraft:item/diamond_sword");
    }

    #[test]
    fn test_material_mapping_resolve_not_found() {
        let mapping = MaterialMapping::default();

        let result = mapping.resolve_fallback_model_path("unknown");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("見つかりません"));
    }
}
