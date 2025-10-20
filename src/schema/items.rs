use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResource {
    pub model: ItemResourceModel,
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
