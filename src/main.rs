use std::path::Path;

use clap::Parser;

mod cli;
mod constants;
mod file_utils;
mod gallery;
mod image_validator;
mod models;
mod processor;

use cli::{Cli, Commands};
use gallery::GalleryGenerator;
use processor::Processor;

fn main() {
    if let Err(err) = run() {
        eprintln!("\n❌ エラー:\n{}", err);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Validate that we're in a resource pack directory (except for generate-gallery)
    match &cli.command {
        Commands::GenerateGallery { .. } => {}
        _ => {
            if !Path::new("assets").exists() {
                anyhow::bail!(
                    "assetsディレクトリが存在しません。\n\
                    Minecraftリソースパックのルートディレクトリで実行してください。"
                );
            }
        }
    }

    cli.validate()?;

    match &cli.command {
        Commands::Add {
            materials,
            custom_model_data,
            path_to_image,
        } => {
            let custom_model_data = cli::get_custom_model_data(custom_model_data, path_to_image)?;
            let normalized_materials = cli::normalize_materials(materials);

            let processor = Processor::new(custom_model_data);
            processor.add_with_texture(&normalized_materials, path_to_image)?;
        }
        Commands::Extend {
            materials,
            custom_model_data,
        } => {
            let custom_model_data = custom_model_data.to_lowercase();
            let normalized_materials = cli::normalize_materials(materials);

            let processor = Processor::new(custom_model_data);
            processor.extend_materials(&normalized_materials)?;
        }
        Commands::GenerateGallery { output } => {
            let generator = GalleryGenerator::new(output.clone());
            generator.generate()?;
        }
    }

    Ok(())
}
