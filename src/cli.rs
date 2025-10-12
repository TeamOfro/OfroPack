use std::path::PathBuf;

use clap::Parser;

/// A simple CLI to add custom model data to a Minecraft resource pack.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The material to add the custom model data to (e.g., diamond_axe)
    #[arg(short, long)]
    pub materials: Vec<String>,

    /// The name for the custom model data
    #[arg(short, long)]
    pub custom_model_data: Option<String>,

    /// The path to the image file for the custom model
    pub path_to_image: PathBuf,
}

impl Cli {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.materials.is_empty() {
            return Err(anyhow::anyhow!(
                "At least one material must be specified using the --materials flag."
            ));
        }

        if !self.path_to_image.exists() {
            return Err(anyhow::anyhow!("Image file does not exist."));
        }

        Ok(())
    }

    pub fn get_custom_model_data(&self) -> anyhow::Result<String> {
        let custom_model_data = match &self.custom_model_data {
            Some(name) => name.to_lowercase(),
            None => self
                .path_to_image
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Failed to get image file name"))?
                .to_string(),
        };

        if !custom_model_data
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return Err(anyhow::anyhow!(
                "Custom model data name must only contain lowercase letters, numbers, and underscores."
            ));
        }

        Ok(custom_model_data)
    }

    pub fn get_normalized_materials(&self) -> Vec<String> {
        self.materials.iter().map(|m| m.to_lowercase()).collect()
    }
}
