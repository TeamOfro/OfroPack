use anyhow::{Context, Result};
use clap::Parser;

use crate::cmd::{Cmd, Run};

#[derive(Debug, clap::Parser)]
pub struct Generates;

impl Run for Generates {
    fn run(&self) -> Result<()> {
        println!("📦 リソースパックを圧縮中...");
        Cmd::try_parse_from(["", "zip"])
            .with_context(|| "'zip' コマンドの解析に失敗しました")?
            .run()?;

        println!("🎨 ギャラリーデータを生成中...");
        Cmd::try_parse_from(["", "models"])
            .with_context(|| "'models' コマンドの解析に失敗しました")?
            .run()?;

        println!("📝 メタデータを生成中...");
        Cmd::try_parse_from(["", "metadata"])
            .with_context(|| "'metadata' コマンドの解析に失敗しました")?
            .run()?;

        Ok(())
    }
}
