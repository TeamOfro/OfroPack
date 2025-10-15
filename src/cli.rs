use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

/// A simple CLI to add custom model data to a Minecraft resource pack.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new custom model with texture
    Add {
        /// The materials to add the custom model data to (e.g., diamond_axe)
        #[arg(short, long, required = true)]
        materials: Vec<String>,

        /// The name for the custom model data
        #[arg(short, long)]
        custom_model_data: Option<String>,

        /// The path to the image file for the custom model
        path_to_image: PathBuf,
    },

    /// Add materials to an existing custom model
    Extend {
        /// The materials to add
        #[arg(short, long, required = true)]
        materials: Vec<String>,

        /// The name of the existing custom model data
        #[arg(short, long, required = true)]
        custom_model_data: String,
    },
}

impl Cli {
    pub fn validate(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Add {
                path_to_image,
                materials,
                ..
            } => {
                if materials.is_empty() {
                    return Err(anyhow::anyhow!(
                        "At least one material must be specified using the --materials flag."
                    ));
                }

                if !path_to_image.exists() {
                    return Err(anyhow::anyhow!(
                        "Image file does not exist: {}",
                        path_to_image.display()
                    ));
                }
            }
            Commands::Extend {
                materials,
                custom_model_data,
            } => {
                if materials.is_empty() {
                    return Err(anyhow::anyhow!(
                        "At least one material must be specified using the --materials flag."
                    ));
                }

                if !custom_model_data
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                {
                    return Err(anyhow::anyhow!(
                        "Custom model data name must only contain lowercase letters, numbers, and underscores."
                    ));
                }
            }
        }
        Ok(())
    }
}

pub fn get_custom_model_data(
    custom_model_data: &Option<String>,
    path_to_image: &Path,
) -> anyhow::Result<String> {
    let custom_model_data = match custom_model_data {
        Some(name) => name.to_lowercase(),
        None => path_to_image
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

pub fn normalize_materials(materials: &[String]) -> Vec<String> {
    materials.iter().map(|m| m.to_lowercase()).collect()
}
