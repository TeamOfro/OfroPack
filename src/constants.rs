use std::{path::PathBuf, str::FromStr};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GithubReaction {
    ThumbsUp,
    ThumbsDown,
    Laugh,
    Confused,
    Heart,
    Hooray,
    Rocket,
    Eyes,
}

impl GithubReaction {
    pub const fn as_str(&self) -> &'static str {
        match self {
            GithubReaction::ThumbsUp => "+1",
            GithubReaction::ThumbsDown => "-1",
            GithubReaction::Laugh => "laugh",
            GithubReaction::Confused => "confused",
            GithubReaction::Heart => "heart",
            GithubReaction::Hooray => "hooray",
            GithubReaction::Rocket => "rocket",
            GithubReaction::Eyes => "eyes",
        }
    }
}

impl FromStr for GithubReaction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+1" => Ok(GithubReaction::ThumbsUp),
            "-1" => Ok(GithubReaction::ThumbsDown),
            "laugh" => Ok(GithubReaction::Laugh),
            "confused" => Ok(GithubReaction::Confused),
            "heart" => Ok(GithubReaction::Heart),
            "hooray" => Ok(GithubReaction::Hooray),
            "rocket" => Ok(GithubReaction::Rocket),
            "eyes" => Ok(GithubReaction::Eyes),
            _ => Err(format!("'{}' is not a valid Github reaction", s)),
        }
    }
}

impl clap::ValueEnum for GithubReaction {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            GithubReaction::ThumbsUp,
            GithubReaction::ThumbsDown,
            GithubReaction::Laugh,
            GithubReaction::Confused,
            GithubReaction::Heart,
            GithubReaction::Hooray,
            GithubReaction::Rocket,
            GithubReaction::Eyes,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        <Self as FromStr>::from_str(&input)
    }
}

impl serde::Serialize for GithubReaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for GithubReaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for GithubReaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
