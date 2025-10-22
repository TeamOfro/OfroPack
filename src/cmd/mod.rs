pub mod add;
pub mod extend;
mod generates;
mod metadata;
mod models;
mod runner;
mod zip;

/// CLI for OfroPack - Minecraft Resource Pack Manager
#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub enum Cmd {
    Add(add::Add),
    Extend(extend::Extend),
    Models(models::Models),
    Metadata(metadata::Metadata),
    Zip(zip::Zip),
    Runner(runner::Runner),
    Generates(generates::Generates),
}

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

impl Run for Cmd {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Add(cmd) => cmd.run(),
            Self::Extend(cmd) => cmd.run(),
            Self::Models(cmd) => cmd.run(),
            Self::Metadata(cmd) => cmd.run(),
            Self::Zip(cmd) => cmd.run(),
            Self::Runner(cmd) => cmd.run(),
            Self::Generates(cmd) => cmd.run(),
        }
    }
}
