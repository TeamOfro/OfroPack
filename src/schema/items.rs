//! アイテムリソース定義
//!
//! Minecraftのアイテムモデル選択機能のスキーマを定義します。

use serde::{Deserialize, Serialize};

/// アイテムリソース（アイテム定義ファイルの最上位構造）
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResource {
    /// モデル選択定義
    pub model: ItemResourceModel,
}

impl ItemResource {
    /// フォールバックモデルを指定して新しいアイテムリソースを作成
    ///
    /// # Examples
    ///
    /// ```
    /// use processor::schema::items::ItemResource;
    ///
    /// let resource = ItemResource::new_with_fallback("minecraft:item/diamond_sword");
    /// ```
    pub fn new_with_fallback(fallback_model: &str) -> Self {
        Self {
            model: ItemResourceModel {
                r#type: "minecraft:select".to_string(),
                property: "minecraft:custom_model_data".to_string(),
                fallback: ItemFallback {
                    r#type: "minecraft:model".to_string(),
                    model: fallback_model.to_string(),
                },
                cases: Vec::new(),
            },
        }
    }

    /// カスタムモデルケースを追加
    pub fn add_case(&mut self, case: ItemCase) {
        self.model.cases.push(case);
    }
}

/// アイテムモデル選択定義
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResourceModel {
    /// 選択タイプ（通常は "minecraft:select"）
    pub r#type: String,
    /// 選択プロパティ（通常は "minecraft:custom_model_data"）
    pub property: String,
    /// デフォルトモデル
    pub fallback: ItemFallback,
    /// カスタムモデルケース一覧
    pub cases: Vec<ItemCase>,
}

/// フォールバックモデル定義
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemFallback {
    /// モデルタイプ（通常は "minecraft:model"）
    pub r#type: String,
    /// モデルパス
    pub model: String,
}

/// カスタムモデルケース
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCase {
    /// カスタムモデルデータ値
    pub when: String,
    /// 使用するモデル
    pub model: ItemCaseModel,
}

/// ケースで使用するモデル定義
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCaseModel {
    /// モデルタイプ（通常は "minecraft:model"）
    pub r#type: String,
    /// モデルパス
    pub model: String,
}

impl ItemCase {
    /// 新しいカスタムモデルケースを作成
    ///
    /// # Examples
    ///
    /// ```
    /// use processor::schema::items::ItemCase;
    ///
    /// let case = ItemCase::new("my_custom_model");
    /// assert_eq!(case.when, "my_custom_model");
    /// ```
    pub fn new(custom_model_data: &str) -> Self {
        Self {
            when: custom_model_data.to_string(),
            model: ItemCaseModel {
                r#type: "minecraft:model".to_string(),
                model: format!("item/{custom_model_data}"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_case_new() {
        let case = ItemCase::new("test_model");
        assert_eq!(case.when, "test_model");
        assert_eq!(case.model.model, "item/test_model");
        assert_eq!(case.model.r#type, "minecraft:model");
    }

    #[test]
    fn test_item_resource_new_with_fallback() {
        let resource = ItemResource::new_with_fallback("minecraft:item/diamond_sword");
        assert_eq!(resource.model.r#type, "minecraft:select");
        assert_eq!(resource.model.property, "minecraft:custom_model_data");
        assert_eq!(
            resource.model.fallback.model,
            "minecraft:item/diamond_sword"
        );
        assert!(resource.model.cases.is_empty());
    }

    #[test]
    fn test_item_resource_add_case() {
        let mut resource = ItemResource::new_with_fallback("minecraft:item/diamond_sword");
        let case = ItemCase::new("custom_sword");

        resource.add_case(case);

        assert_eq!(resource.model.cases.len(), 1);
        assert_eq!(resource.model.cases[0].when, "custom_sword");
    }
}
