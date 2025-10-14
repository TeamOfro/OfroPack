use std::path::Path;

use clap::Parser;

mod cli;
mod constants;
mod file_utils;
mod models;
mod processor;

use cli::{Cli, Commands};
use processor::Processor;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !Path::new("assets").exists() {
        return Err(anyhow::anyhow!(
            "Assets directory does not exist. Please run this command in the root directory of a Minecraft resource pack."
        ));
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
    }

    Ok(())
}
