//! アイテムモデル定義
//!
//! Minecraftのアイテムモデルファイルのスキーマを定義します。

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::ItemModelParent;

/// アイテムモデル
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemModel {
    /// 親モデル（2D描画スタイル）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ItemModelParent>,
    /// テクスチャマッピング
    pub textures: Textures,
}

/// テクスチャマッピング
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Textures(BTreeMap<String, String>);

impl ItemModel {
    /// 新しい2Dアイテムモデルを作成
    ///
    /// # Examples
    ///
    /// ```
    /// use processor::schema::models::ItemModel;
    /// use processor::types::ItemModelParent;
    ///
    /// let model = ItemModel::new(ItemModelParent::Handheld, "my_sword");
    /// ```
    pub fn new(parent: ItemModelParent, custom_model_data: &str) -> Self {
        let mut textures = Textures::default();
        textures.add_custom(custom_model_data);
        Self {
            parent: Some(parent),
            textures,
        }
    }
}

impl Textures {
    /// カスタムテクスチャを追加（layer0）
    pub fn add_custom(&mut self, custom_model_data: &str) {
        self.0
            .insert("layer0".to_string(), format!("item/{custom_model_data}"));
    }

    /// レイヤーテクスチャを追加
    fn add_layer(&mut self, custom_model_data: &str, layer_number: usize) {
        let key = layer_number.to_string();
        self.0
            .insert(key, format!("item/{custom_model_data}/{layer_number}"));
    }

    /// 既存のテクスチャを上書きして3Dモデル用に設定
    ///
    /// テクスチャマッピングをクリアし、レイヤー番号のキーを持つ
    /// テクスチャを再設定します。
    ///
    /// # Returns
    ///
    /// 上書きされたレイヤー数
    pub fn overwrite(&mut self, custom_model_data: &str) -> usize {
        let keys = self
            .0
            .keys()
            .filter(|k| k.chars().all(char::is_numeric))
            .count();

        self.0.clear();

        for i in 0..keys {
            self.add_layer(custom_model_data, i);
        }

        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_model_new() {
        let model = ItemModel::new(ItemModelParent::Handheld, "test_model");

        assert!(model.parent.is_some());
        assert_eq!(model.parent.unwrap(), ItemModelParent::Handheld);
    }

    #[test]
    fn test_textures_add_custom() {
        let mut textures = Textures::default();
        textures.add_custom("my_texture");

        assert_eq!(textures.0.get("layer0").unwrap(), "item/my_texture");
    }

    #[test]
    fn test_textures_add_layer() {
        let mut textures = Textures::default();
        textures.add_layer("my_model", 0);
        textures.add_layer("my_model", 1);

        assert_eq!(textures.0.get("0").unwrap(), "item/my_model/0");
        assert_eq!(textures.0.get("1").unwrap(), "item/my_model/1");
    }

    #[test]
    fn test_textures_overwrite() {
        let mut textures = Textures::default();
        textures.0.insert("0".to_string(), "old/0".to_string());
        textures.0.insert("1".to_string(), "old/1".to_string());
        textures.0.insert("2".to_string(), "old/2".to_string());

        let count = textures.overwrite("new_model");

        assert_eq!(count, 3);
        assert_eq!(textures.0.get("0").unwrap(), "item/new_model/0");
        assert_eq!(textures.0.get("1").unwrap(), "item/new_model/1");
        assert_eq!(textures.0.get("2").unwrap(), "item/new_model/2");
    }
}
