use std::path::Path;

use clap::Parser;

mod cli;
mod constants;
mod file_utils;
mod models;
mod processor;

use cli::Cli;
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

    let custom_model_data = cli.get_custom_model_data()?;
    let materials = cli.get_normalized_materials();

    let processor = Processor::new(custom_model_data, cli.path_to_image);

    processor.validate()?;
    processor.prepare()?;

    for material in materials {
        processor.process_material(&material)?;
    }

    processor.finalize()?;

    Ok(())
}
