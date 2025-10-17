use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationInfo {
    pub frametime: NonZeroU32,
    pub frame_count: u32,
}

impl AnimationInfo {
    pub fn new(frametime: NonZeroU32) -> Self {
        Self {
            frametime,
            frame_count: 0, // Will be set after image validation
        }
    }

    pub fn with_frame_count(mut self, frame_count: u32) -> Self {
        self.frame_count = frame_count;
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemOverride {
    pub model: ItemModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemModel {
    pub r#type: String,
    pub property: String,
    pub fallback: Fallback,
    pub cases: Vec<Case>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fallback {
    pub r#type: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Case {
    pub when: String,
    pub model: CaseModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaseModel {
    pub r#type: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelFile {
    pub parent: String,
    pub textures: Textures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Textures {
    pub layer0: String,
}

impl ItemOverride {
    pub fn new(material: &str) -> Self {
        Self {
            model: ItemModel {
                r#type: "minecraft:select".to_string(),
                property: "minecraft:custom_model_data".to_string(),
                fallback: Fallback {
                    r#type: "minecraft:model".to_string(),
                    model: format!("minecraft:item/{}", material),
                },
                cases: Vec::new(),
            },
        }
    }

    pub fn add_case(&mut self, custom_model_data: &str) {
        let new_case = Case {
            when: custom_model_data.to_string(),
            model: CaseModel {
                r#type: "minecraft:model".to_string(),
                model: format!("item/{}", custom_model_data),
            },
        };
        self.model.cases.push(new_case);
    }
}

impl ModelFile {
    pub fn new(custom_model_data: &str) -> Self {
        Self {
            parent: "minecraft:item/generated".to_string(),
            textures: Textures {
                layer0: format!("minecraft:item/{}", custom_model_data),
            },
        }
    }
}
