pub mod add;
pub mod extend;
mod metadata;
mod models;
mod runner;
mod zip;
mod generates;

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
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Extend(cmd) => cmd.run(),
            Cmd::Models(cmd) => cmd.run(),
            Cmd::Metadata(cmd) => cmd.run(),
            Cmd::Zip(cmd) => cmd.run(),
            Cmd::Runner(cmd) => cmd.run(),
            Cmd::Generates(cmd) => cmd.run(),
        }
    }
}
