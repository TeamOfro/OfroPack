use std::{path::PathBuf, process::Command};

use anyhow::Context;

/// 📦 リソースパックをZip化
///
/// assetsディレクトリとpack.mcmetaをZipファイルにまとめます。
/// 生成されたZipファイルはMinecraftのリソースパックとして使用できます。
#[derive(clap::Parser, Debug)]
#[command(
    about = "リソースパックをZip化",
    long_about = "assetsディレクトリとpack.mcmetaをZipファイルにまとめます。\n\n\
                  生成されたZipファイルは、Minecraftのリソースパックとして\n\
                  そのまま使用できます。"
)]
pub struct Zip {
    /// 出力Zipファイルパス
    #[arg(
        short,
        long,
        default_value = "OfroPack.zip",
        value_name = "OUTPUT",
        help = "出力Zipファイルのパス"
    )]
    pub output: PathBuf,

    /// Zipに含めるファイル/ディレクトリのリスト
    #[arg(
        short,
        long,
        default_values = &["assets/", "pack.mcmeta", "pack.png"],
        value_name = "FILES",
        help = "含めるファイル/ディレクトリ"
    )]
    pub files: Vec<String>,
}

impl super::Run for Zip {
    fn run(&self) -> anyhow::Result<()> {
        println!("\n📦 リソースパック圧縮を開始します...\n");

        // Validate that files exist
        println!("🔍 ファイルを確認中...");
        for file in &self.files {
            let path = std::path::Path::new(file);
            if !path.exists() {
                anyhow::bail!("❌ ファイルが見つかりません: {}", file);
            }
            println!("  ✓ {}", file);
        }

        // Remove existing zip if it exists
        if self.output.exists() {
            println!("\n🗑️  既存のZipファイルを削除中...");
            std::fs::remove_file(&self.output).with_context(|| {
                format!("既存のZipファイルの削除に失敗: {}", self.output.display())
            })?;
        }

        {
            println!("\n🗜️  圧縮中...");
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
                    "❌ Zipの作成に失敗:\n{}",
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

        // Format file size
        let size_str = if size > 1024 * 1024 {
            format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
        } else if size > 1024 {
            format!("{:.2} KB", size as f64 / 1024.0)
        } else {
            format!("{} bytes", size)
        };

        println!("\n✅ リソースパックを作成しました！");
        println!("  📁 出力: {}", self.output.display());
        println!("  📊 サイズ: {}\n", size_str);

        Ok(())
    }
}
