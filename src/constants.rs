use std::path::PathBuf;

pub struct Paths;

impl Paths {
    fn root() -> PathBuf {
        #[cfg(test)]
        {
            // In tests, use TEST_ROOT environment variable if set
            if let Ok(test_root) = std::env::var("TEST_ROOT") {
                return PathBuf::from(test_root);
            }
        }
        PathBuf::from(".")
    }

    pub const ITEMS: &str = "assets/minecraft/items";
    pub const MODELS: &str = "assets/minecraft/models/item";
    pub const TEXTURES: &str = "assets/minecraft/textures/item";

    pub fn item_path(material: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.json", Self::ITEMS, material))
    }

    pub fn model_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.json", Self::MODELS, custom_model_data))
    }

    pub fn texture_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.png", Self::TEXTURES, custom_model_data))
    }
}
