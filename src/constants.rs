use std::path::PathBuf;

use crate::constcat;

pub const REPO_OWNER: &str = "TeamOfro";
pub const REPO_NAME: &str = "OfroPack";
pub const REPO_URL: &str = constcat!("https://github.com/", REPO_OWNER, "/", REPO_NAME);

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
    pub const PREVIEWS: &str = "previews";

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

    pub fn animation_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!(
            "{}/{}.png.mcmeta",
            Self::TEXTURES,
            custom_model_data
        ))
    }

    pub fn relative_preview_path(custom_model_data: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}.png", Self::PREVIEWS, custom_model_data))
    }

    pub fn preview_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(Self::relative_preview_path(custom_model_data))
    }
}

pub fn is_snake_case(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit() => (),
        _ => return false,
    }
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return false;
        }
    }
    true
}

pub fn should_snake_case(s: &str) -> anyhow::Result<()> {
    if is_snake_case(s) {
        Ok(())
    } else {
        anyhow::bail!("'{}'はスネークケースで指定してください。", s);
    }
}
