use crate::config::REPO_URL;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::cmd::Run;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectMetadata {
    version: String,
    sha1: String,
    size: u64,
    commit: String,
    updated_at: String,
    latest_pr: Option<LatestPr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestPr {
    number: u32,
    title: String,
    url: String,
}

#[derive(clap::Parser, Debug)]
pub struct Metadata {
    /// 出力ファイルパス (デフォルト: metadata.json)
    #[arg(short, long, default_value = "metadata.json")]
    pub output: PathBuf,

    /// ZipファイルのパスSHA1計算とサイズ取得に使用
    #[arg(short, long, default_value = "OfroPack.zip")]
    pub zip: PathBuf,
}

impl Run for Metadata {
    fn run(&self) -> Result<()> {
        println!("📝 メタデータを生成中...");

        // Check if zip file exists
        if !self.zip.exists() {
            anyhow::bail!("Zipファイルが見つかりません: {}", self.zip.display());
        }

        // Get SHA1
        let sha1 = self.calculate_sha1()?;
        println!("  ✓ SHA1: {}", sha1);

        // Get file size
        let size = fs::metadata(&self.zip)
            .context(format!(
                "Zipファイルのメタデータ取得に失敗: {}",
                self.zip.display()
            ))?
            .len();
        println!("  ✓ サイズ: {} bytes", size);

        // Get current commit hash
        let commit = self.get_current_commit()?;
        println!("  ✓ コミット: {}", commit);

        // Get current timestamp
        let updated_at = chrono::Utc::now().to_rfc3339();

        // Generate version string
        let version = chrono::Utc::now().format("%Y%m%d-%H%M%S").to_string();

        // Get latest PR
        let latest_pr = self.get_latest_merged_pr()?;
        if let Some(ref pr) = latest_pr {
            println!("  ✓ 最新PR: #{} - {}", pr.number, pr.title);
        }

        let metadata = CollectMetadata {
            version,
            sha1,
            size,
            commit,
            updated_at,
            latest_pr,
        };

        // Write to file
        self.write_metadata(&metadata)?;

        println!("\n✅ メタデータを生成しました: {}", self.output.display());

        Ok(())
    }
}

impl Metadata {
    /// Calculate SHA1 hash of the zip file
    fn calculate_sha1(&self) -> Result<String> {
        let output = Command::new("sha1sum")
            .arg(&self.zip)
            .output()
            .context("sha1sumコマンドの実行に失敗")?;

        if !output.status.success() {
            anyhow::bail!(
                "SHA1の計算に失敗: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let sha1 = String::from_utf8(output.stdout)
            .context("SHA1出力のパースに失敗")?
            .split_whitespace()
            .next()
            .context("SHA1の抽出に失敗")?
            .to_string();

        Ok(sha1)
    }

    /// Get current git commit hash
    fn get_current_commit(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .context("gitコマンドの実行に失敗")?;

        if !output.status.success() {
            return Ok("unknown".to_string());
        }

        Ok(String::from_utf8(output.stdout)
            .context("コミットハッシュのパースに失敗")?
            .trim()
            .to_string())
    }

    /// Get the latest merged PR from git log
    fn get_latest_merged_pr(&self) -> Result<Option<LatestPr>> {
        let output = Command::new("git")
            .args([
                "log",
                "--merges",
                "--format=%H|%s|%b",
                "-50",
                "--grep=^Merge pull request",
            ])
            .output();

        if let Ok(output) = output
            && output.status.success()
        {
            let log = String::from_utf8_lossy(&output.stdout);

            for line in log.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 2 {
                    continue;
                }

                let subject = parts[1];

                // Parse "Merge pull request #123 from ..."
                if let Some(pr_part) = subject.strip_prefix("Merge pull request #")
                    && let Some(number_end) = pr_part.find(' ')
                {
                    let number = pr_part[..number_end].to_string();
                    let number: u32 = match number.parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    // Get title from body (3rd part)
                    let title = if parts.len() > 2 && !parts[2].is_empty() {
                        parts[2].lines().next().unwrap_or("No title").to_string()
                    } else {
                        "No title".to_string()
                    };

                    let url = format!("{REPO_URL}/pull/{number}");

                    return Ok(Some(LatestPr { number, title, url }));
                }
            }
        }

        Ok(None)
    }

    /// Write metadata to JSON file
    fn write_metadata(&self, metadata: &CollectMetadata) -> Result<()> {
        // Create parent directory if needed
        if let Some(parent) = self.output.parent() {
            fs::create_dir_all(parent)
                .context(format!("ディレクトリの作成に失敗: {}", parent.display()))?;
        }

        let json = serde_json::to_string_pretty(metadata).context("JSONのシリアライズに失敗")?;

        fs::write(&self.output, json).context(format!(
            "ファイルの書き込みに失敗: {}",
            self.output.display()
        ))?;

        Ok(())
    }
}
