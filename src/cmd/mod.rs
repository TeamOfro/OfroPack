mod add;
mod extend;
mod zip;

/// CLI for OfroPack - Minecraft Resource Pack Manager
#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub enum Cmd {
    Add(add::Add),
    Extend(extend::Extend),
    // GenerateGallery(GenerateGallery),
    // GenerateMetadata(GenerateMetadata),
    Zip(zip::Zip),
    // Generates(Generates),
    // Runner(Runner),
}

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

impl Run for Cmd {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Extend(cmd) => cmd.run(),
            // Cmd::GenerateGallery(cmd) => cmd.run(),
            // Cmd::GenerateMetadata(cmd) => cmd.run(),
            Cmd::Zip(cmd) => cmd.run(),
            // Cmd::Generates(cmd) => cmd.run(),
            // Cmd::Runner(cmd) => cmd.run(),
        }
    }
}
