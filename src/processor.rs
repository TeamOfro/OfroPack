use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::constants::Paths;
use crate::file_utils::{create_parent_dir_all, read_json, write_json};
use crate::models::{ItemOverride, ModelFile};

pub struct Processor {
    pub custom_model_data: String,
    pub image_path: PathBuf,
    pub model_path: PathBuf,
    pub texture_path: PathBuf,
}

impl Processor {
    pub fn new(custom_model_data: String, image_path: PathBuf) -> Self {
        let model_path = Paths::model_path(&custom_model_data);
        let texture_path = Paths::texture_path(&custom_model_data);

        Self {
            custom_model_data,
            image_path,
            model_path,
            texture_path,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.model_path.exists() {
            return Err(anyhow::anyhow!(
                "Model file already exists at '{}'. Please choose a different custom model data name.",
                self.model_path.display()
            ));
        }
        Ok(())
    }

    pub fn prepare(&self) -> anyhow::Result<()> {
        create_parent_dir_all(&self.model_path)?;
        create_parent_dir_all(&self.texture_path)?;
        Ok(())
    }

    pub fn process_material(&self, material: &str) -> anyhow::Result<()> {
        let item_path = Paths::item_path(material);
        create_parent_dir_all(&item_path)?;

        self.update_item_override_file(&item_path, material)?;

        println!(
            "Adding custom model data '{}' to '{}'.",
            self.custom_model_data, material
        );

        Ok(())
    }

    pub fn finalize(&self) -> anyhow::Result<()> {
        self.create_model_file()?;
        std::fs::copy(&self.image_path, &self.texture_path)
            .context("Failed to copy image to texture path")?;
        Ok(())
    }

    fn update_item_override_file(&self, item_path: &Path, material: &str) -> anyhow::Result<()> {
        let mut item_override = if item_path.exists() {
            read_json(item_path).context("Failed to read existing item JSON")?
        } else {
            ItemOverride::new(material)
        };

        item_override.add_case(&self.custom_model_data);

        write_json(item_path, &item_override).context("Failed to write updated item JSON")
    }

    fn create_model_file(&self) -> anyhow::Result<()> {
        let model_file = ModelFile::new(&self.custom_model_data);
        write_json(&self.model_path, &model_file).context("Failed to write model JSON")
    }
}
