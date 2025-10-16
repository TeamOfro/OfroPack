mod add;
mod cmd;
mod extend;
mod gallery;
mod runner;

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
            Cmd::Runner(cmd) => cmd.run(),
        }
    }
}
