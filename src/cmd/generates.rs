use anyhow::{Context, Result};
use clap::Parser;

use crate::Cmd;
use crate::cmd::{Generates, Run};

impl Run for Generates {
    fn run(&self) -> Result<()> {
        Cmd::try_parse_from(["", "generate-zip"])
            .with_context(|| "Failed to parse 'generate-zip' command")?
            .run()?;
        Cmd::try_parse_from(["", "generate-gallery"])
            .with_context(|| "Failed to parse 'generate-gallery' command")?
            .run()?;
        Cmd::try_parse_from(["", "generate-metadata"])
            .with_context(|| "Failed to parse 'generate-metadata' command")?
            .run()?;
        Ok(())
    }
}
