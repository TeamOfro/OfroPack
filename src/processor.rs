use std::path::Path;

use anyhow::Context;

use crate::constants::Paths;
use crate::file_utils::{create_parent_dir_all, read_json, write_json};
use crate::models::{ItemOverride, ModelFile};

pub struct Processor {
    pub custom_model_data: String,
}

impl Processor {
    pub fn new(custom_model_data: String) -> Self {
        Self { custom_model_data }
    }

    /// Add a new custom model with texture
    pub fn add_with_texture(&self, materials: &[String], image_path: &Path) -> anyhow::Result<()> {
        let model_path = Paths::model_path(&self.custom_model_data);
        let texture_path = Paths::texture_path(&self.custom_model_data);

        // Validate that model doesn't already exist
        if model_path.exists() {
            return Err(anyhow::anyhow!(
                "Model file already exists at '{}'. Use 'extend' command to add more materials.",
                model_path.display()
            ));
        }

        // Prepare directories
        create_parent_dir_all(&model_path)?;
        create_parent_dir_all(&texture_path)?;

        // Process materials
        for material in materials {
            self.add_material_to_item(material)?;
        }

        // Create model file
        self.create_model_file(&model_path)?;

        // Copy texture
        std::fs::copy(image_path, &texture_path).context("Failed to copy image to texture path")?;

        println!(
            "✓ Created custom model '{}' with {} material(s)",
            self.custom_model_data,
            materials.len()
        );

        Ok(())
    }

    /// Add materials to an existing custom model
    pub fn extend_materials(&self, materials: &[String]) -> anyhow::Result<()> {
        let model_path = Paths::model_path(&self.custom_model_data);
        let texture_path = Paths::texture_path(&self.custom_model_data);

        // Validate that model already exists
        if !model_path.exists() {
            return Err(anyhow::anyhow!(
                "Model file does not exist at '{}'. Use 'add' command to create a new custom model.",
                model_path.display()
            ));
        }

        if !texture_path.exists() {
            return Err(anyhow::anyhow!(
                "Texture file does not exist at '{}'. The custom model data may be incomplete.",
                texture_path.display()
            ));
        }

        // Process materials
        for material in materials {
            self.add_material_to_item(material)?;
        }

        println!(
            "✓ Extended custom model '{}' with {} material(s)",
            self.custom_model_data,
            materials.len()
        );

        Ok(())
    }

    /// Add custom model data reference to a material's item file
    fn add_material_to_item(&self, material: &str) -> anyhow::Result<()> {
        let item_path = Paths::item_path(material);
        create_parent_dir_all(&item_path)?;

        let mut item_override = if item_path.exists() {
            read_json(&item_path).context("Failed to read existing item JSON")?
        } else {
            ItemOverride::new(material)
        };

        // Check if this custom model data is already added to avoid duplicates
        if item_override
            .model
            .cases
            .iter()
            .any(|c| c.when == self.custom_model_data)
        {
            println!(
                "⚠ Custom model '{}' already exists in material '{}', skipping",
                self.custom_model_data, material
            );
            return Ok(());
        }

        item_override.add_case(&self.custom_model_data);
        write_json(&item_path, &item_override).context("Failed to write updated item JSON")?;

        println!("  • Added to material '{}'", material);

        Ok(())
    }

    /// Create the model JSON file
    fn create_model_file(&self, model_path: &Path) -> anyhow::Result<()> {
        let model_file = ModelFile::new(&self.custom_model_data);
        write_json(model_path, &model_file).context("Failed to write model JSON")
    }
}
