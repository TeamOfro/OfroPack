use std::{path::PathBuf, process::Command};

use anyhow::Context;

/// リソースパックをZipに圧縮する
#[derive(clap::Parser, Debug)]
pub struct Zip {
    /// 出力Zipファイルのパス (デフォルト: OfroPack.zip)
    #[arg(short, long, default_value = "OfroPack.zip")]
    pub output: PathBuf,

    /// Zipに含めるファイル/ディレクトリのリスト
    #[arg(short, long, default_values = &["assets/", "pack.mcmeta", "pack.png"])]
    pub files: Vec<String>,
}

impl super::Run for Zip {
    fn run(&self) -> anyhow::Result<()> {
        println!("📦 リソースパックを圧縮中...");

        // Validate that files exist
        for file in &self.files {
            let path = std::path::Path::new(file);
            if !path.exists() {
                anyhow::bail!("ファイルが見つかりません: {}", file);
            }
        }

        // Remove existing zip if it exists
        if self.output.exists() {
            std::fs::remove_file(&self.output).with_context(|| {
                format!("既存のZipファイルの削除に失敗: {}", self.output.display())
            })?;
        }

        {
            // Build zip command
            let mut cmd = Command::new("zip");
            cmd.arg("-r").arg(&self.output);

            // Add all files
            for file in &self.files {
                cmd.arg(file);
            }

            // Execute zip command
            let output = cmd.output().context("zipコマンドの実行に失敗")?;

            if !output.status.success() {
                anyhow::bail!(
                    "Zipの作成に失敗:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        // Get file size
        let size = std::fs::metadata(&self.output)
            .with_context(|| {
                format!(
                    "Zipファイルのメタデータ取得に失敗: {}",
                    self.output.display()
                )
            })?
            .len();

        println!("\n✅ リソースパックを作成しました");
        println!("  出力: {}", self.output.display());
        println!("  サイズ: {} bytes", size);

        Ok(())
    }
}
