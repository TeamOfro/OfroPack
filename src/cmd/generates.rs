use anyhow::{Context, Result};
use clap::Parser;

use crate::cmd::{Cmd, Run};

#[derive(Debug, clap::Parser)]
pub struct Generates;

impl Run for Generates {
    fn run(&self) -> Result<()> {
        println!("📦 リソースパックを圧縮中...");
        Cmd::try_parse_from(["", "zip"])
            .with_context(|| "Failed to parse 'zip' command")?
            .run()?;

        println!("🎨 ギャラリーデータを生成中...");
        Cmd::try_parse_from(["", "models"])
            .with_context(|| "Failed to parse 'models' command")?
            .run()?;

        println!("📝 メタデータを生成中...");
        Cmd::try_parse_from(["", "metadata"])
            .with_context(|| "Failed to parse 'metadata' command")?
            .run()?;

        Ok(())
    }
}
