use std::path::PathBuf;

pub struct Paths;

impl Paths {
    pub const ITEMS: &str = "assets/minecraft/items";
    pub const MODELS: &str = "assets/minecraft/models/item";
    pub const TEXTURES: &str = "assets/minecraft/textures/item";

    pub fn item_path(material: &str) -> PathBuf {
        format!("{}/{}.json", Self::ITEMS, material).into()
    }

    pub fn model_path(custom_model_data: &str) -> PathBuf {
        format!("{}/{}.json", Self::MODELS, custom_model_data).into()
    }

    pub fn texture_path(custom_model_data: &str) -> PathBuf {
        format!("{}/{}.png", Self::TEXTURES, custom_model_data).into()
    }
}
