use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;
use ureq::Agent;

use crate::pipeline::image_validator::ImageValidator;

/// Image downloader and validator
pub struct ImageDownloader {
    client: Agent,
}

impl ImageDownloader {
    pub fn new() -> Result<Self> {
        let client = Agent::config_builder()
            .user_agent("OfroPack-Image-Downloader")
            .build()
            .into();

        Ok(Self { client })
    }

    /// Download image from URL and validate it's a PNG
    pub fn download(&self, url: &str, output_path: &Path) -> Result<()> {
        println!("画像をダウンロード中: {}", url);

        let mut response = self
            .client
            .get(url)
            .call()
            .context("画像のダウンロードに失敗しました")?;

        let status = response.status();

        if !status.is_success() {
            bail!(
                "画像のダウンロードに失敗しました（HTTPステータス: {}）",
                status
            );
        }

        let bytes = response
            .body_mut()
            .read_to_vec()
            .context("レスポンスの読み取りに失敗しました")?;

        // Save to file
        fs::write(output_path, &bytes).with_context(|| {
            format!(
                "ファイルの書き込みに失敗しました: {}",
                output_path.display()
            )
        })?;

        // Validate it's a PNG using image crate
        ImageValidator::new_png(output_path)?;

        println!("✓ 画像のダウンロードと検証が完了しました");
        Ok(())
    }
}
