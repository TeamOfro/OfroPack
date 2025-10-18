mod add;
mod generates;
mod cmd;
mod extend;
mod gallery;
mod metadata;
mod runner;
mod zip;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Cmd {
    fn run(&self) -> Result<()> {
        match self {
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Extend(cmd) => cmd.run(),
            Cmd::GenerateGallery(cmd) => cmd.run(),
            Cmd::GenerateMetadata(cmd) => cmd.run(),
            Cmd::GenerateZip(cmd) => cmd.run(),
            Cmd::Runner(cmd) => cmd.run(),
            Cmd::Generates(cmd) => cmd.run(),
        }
    }
}
