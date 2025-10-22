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
        PathBuf::new()
    }

    pub const ASSETS: &str = "assets";
    pub const ITEMS: &str = "assets/minecraft/items";
    pub const MODELS: &str = "assets/minecraft/models/item";
    pub const TEXTURES: &str = "assets/minecraft/textures/item";
    pub const PREVIEWS: &str = "preview";

    pub const ITEMS_TEXTURES: &str = "items_textures.json";

    pub fn assets_path() -> PathBuf {
        Self::root().join(Self::ASSETS)
    }

    pub fn item_path(material: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.json", Self::ITEMS, material))
    }

    pub fn model_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.json", Self::MODELS, custom_model_data))
    }

    pub fn texture_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.png", Self::TEXTURES, custom_model_data))
    }

    pub fn texture_path_dir(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{}", Self::TEXTURES, custom_model_data))
    }

    pub fn texture_layer_path(custom_model_data: &str, layer: usize) -> PathBuf {
        Self::root().join(format!(
            "{}/{}/{}.png",
            Self::TEXTURES,
            custom_model_data,
            layer
        ))
    }

    pub fn animation_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!(
            "{}/{}.png.mcmeta",
            Self::TEXTURES,
            custom_model_data
        ))
    }

    pub fn preview_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{}.png", Self::PREVIEWS, custom_model_data))
    }
}
