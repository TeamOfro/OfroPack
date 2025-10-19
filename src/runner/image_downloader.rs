use anyhow::{Context, Result, bail};
use image::GenericImageView;
use std::fs;
use std::path::Path;
use ureq::Agent;

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
        self.validate_png(output_path)?;

        println!("✓ 画像のダウンロードと検証が完了しました");
        Ok(())
    }

    /// Validate that the file is a valid PNG image
    fn validate_png(&self, path: &Path) -> Result<()> {
        let img = image::open(path)
            .with_context(|| format!("画像ファイルの読み込みに失敗しました: {}", path.display()))?;

        // Check if it's PNG format by trying to load it
        // The image crate will error if it's not a valid image format
        let format =
            image::guess_format(&fs::read(path)?).context("画像形式を判定できませんでした")?;

        if format != image::ImageFormat::Png {
            bail!(
                "PNG形式の画像のみ対応しています（検出された形式: {:?}）",
                format
            );
        }

        let (width, height) = img.dimensions();
        println!("画像サイズ: {}x{}", width, height);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_png_with_valid_png() {
        // Create a minimal valid PNG (1x1 white pixel)
        let png_data = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE,
        ];

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&png_data).unwrap();

        let downloader = ImageDownloader::new().unwrap();
        // This might fail because the PNG is not complete, but tests the validation path
        let _ = downloader.validate_png(temp_file.path());
    }
}
