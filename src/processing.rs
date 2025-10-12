use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

// Structs for the item override JSON (e.g., diamond_axe.json)
#[derive(Serialize, Deserialize, Debug)]
struct ItemOverride {
    model: ItemModel,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemModel {
    r#type: String,
    property: String,
    fallback: Fallback,
    cases: Vec<Case>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Fallback {
    r#type: String,
    model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Case {
    when: String,
    model: CaseModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CaseModel {
    r#type: String,
    model: String,
}

// Structs for the model JSON (e.g., bone_of_kenkoukotsu.json)
#[derive(Serialize, Deserialize, Debug)]
struct ModelFile {
    parent: String,
    textures: Textures,
}

#[derive(Serialize, Deserialize, Debug)]
struct Textures {
    layer0: String,
}

#[derive(Debug)]
pub struct Processing {
    pub material: String,
    pub custom_model_data: String,
    pub image_path: PathBuf,

    pub item_path: PathBuf,
    pub model_path: PathBuf,
    pub texture_path: PathBuf,
}

impl Processing {
    pub fn run(self) -> anyhow::Result<()> {
        self.update_item_override_file()?;
        self.create_model_file()?;

        std::fs::copy(&self.image_path, &self.texture_path)
            .context("Failed to copy image to texture path")?;

        Ok(())
    }

    fn update_item_override_file(&self) -> anyhow::Result<()> {
        let mut item_override = if self.item_path.exists() {
            let file_content = std::fs::read_to_string(&self.item_path)
                .context("Failed to read existing item JSON")?;
            serde_json::from_str(&file_content).context("Failed to parse existing item JSON")?
        } else {
            ItemOverride {
                model: ItemModel {
                    r#type: "minecraft:select".to_string(),
                    property: "minecraft:custom_model_data".to_string(),
                    fallback: Fallback {
                        r#type: "minecraft:model".to_string(),
                        model: format!("minecraft:item/{}", self.material),
                    },
                    cases: Vec::new(),
                },
            }
        };

        let new_case = Case {
            when: self.custom_model_data.clone(),
            model: CaseModel {
                r#type: "minecraft:model".to_string(),
                model: format!("item/{}/{}", self.material, self.custom_model_data),
            },
        };

        item_override.model.cases.push(new_case);

        write_json(&self.item_path, &item_override)
            .context("Failed to write updated item JSON")
    }

    fn create_model_file(&self) -> anyhow::Result<()> {
        let model_file = ModelFile {
            parent: "minecraft:item/generated".to_string(),
            textures: Textures {
                layer0: format!("minecraft:item/{}/{}", self.material, self.custom_model_data),
            },
        };

        write_json(&self.model_path, &model_file).context("Failed to write model JSON")
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(value).context("Failed to serialize to JSON")?;
    std::fs::write(path, json_string).context("Failed to write to file")
}