use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::Parser;

mod processing;

/// A simple CLI to add custom model data to a Minecraft resource pack.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The material to add the custom model data to (e.g., diamond_axe)
    material: String,
    /// The name for the custom model data
    custom_model_data: String,
    /// The path to the image file for the custom model
    path_to_image: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

struct Paths;

impl Paths {
    const ITEMS: &str = "assets/minecraft/items";
    const MODELS: &str = "assets/minecraft/models/item";
    const TEXTURES: &str = "assets/minecraft/textures/item";

    fn item_path(material: &str) -> PathBuf {
        format!("{}/{}.json", Self::ITEMS, material).into()
    }

    fn model_path(material: &str, custom_model_data: &str) -> PathBuf {
        format!("{}/{}/{}.json", Self::MODELS, material, custom_model_data).into()
    }

    fn texture_path(material: &str, custom_model_data: &str) -> PathBuf {
        format!("{}/{}/{}.png", Self::TEXTURES, material, custom_model_data).into()
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !Path::new("assets").exists() {
        return Err(anyhow::anyhow!(
            "Assets directory does not exist. Please run this command in the root directory of a Minecraft resource pack."
        ));
    }

    if !cli.path_to_image.exists() {
        return Err(anyhow::anyhow!("Image file does not exist."));
    }

    let material = cli.material.to_lowercase();
    let custom_model_data = cli.custom_model_data.to_lowercase();

    let item_path = Paths::item_path(&material);
    let model_path = Paths::model_path(&material, &custom_model_data);
    let texture_path = Paths::texture_path(&material, &custom_model_data);

    if model_path.exists() {
        return Err(anyhow::anyhow!(
            "Model file already exists at '{}'. Please choose a different custom model data name.",
            model_path.display()
        ));
    }

    create_parent_dir_all(&item_path)?;
    create_parent_dir_all(&model_path)?;
    create_parent_dir_all(&texture_path)?;

    let processor = processing::Processing {
        material,
        custom_model_data,
        image_path: cli.path_to_image,
        item_path,
        model_path,
        texture_path,
    };

    processor.run()?;

    println!(
        "Successfully added custom model data '{}' to '{}'.",
        cli.custom_model_data, cli.material
    );

    Ok(())
}

fn create_parent_dir_all(path: &Path) -> anyhow::Result<()> {
    let parent = path.parent().context("Failed to get parent directory")?;
    if !parent.exists() {
        std::fs::create_dir_all(parent).context("Failed to create parent directories")?;
    }
    Ok(())
}

