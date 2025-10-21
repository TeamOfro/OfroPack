use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResource {
    pub model: ItemResourceModel,
}

impl ItemResource {
    pub fn new(material: &str) -> Self {
        Self {
            model: ItemResourceModel {
                r#type: "minecraft:select".to_string(),
                property: "minecraft:custom_model_data".to_string(),
                fallback: ItemFallback {
                    r#type: "minecraft:model".to_string(),
                    model: format!("minecraft:item/{}", material),
                },
                cases: Vec::new(),
            },
        }
    }

    pub fn add_case(&mut self, case: ItemCase) {
        self.model.cases.push(case);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResourceModel {
    pub r#type: String,
    pub property: String,
    pub fallback: ItemFallback,
    pub cases: Vec<ItemCase>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemFallback {
    pub r#type: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCase {
    pub when: String,
    pub model: ItemCaseModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCaseModel {
    pub r#type: String,
    pub model: String,
}

impl ItemCase {
    pub fn new(custom_model_data: &str) -> Self {
        Self {
            when: custom_model_data.to_string(),
            model: ItemCaseModel {
                r#type: "minecraft:model".to_string(),
                model: format!("item/{}", custom_model_data),
            },
        }
    }
}
