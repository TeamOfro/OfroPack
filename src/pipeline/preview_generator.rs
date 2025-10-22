use anyhow::{Context, Result};
use image::{ImageFormat, ImageReader};
use std::path::{Path, PathBuf};

use crate::paths::Paths;

pub struct PreviewGenerator;

impl PreviewGenerator {
    /// Generate 256x256 preview from source texture with nearest-neighbor interpolation
    pub fn generate(source: &Path, model_name: &str) -> Result<PathBuf> {
        if !source.exists() {
            anyhow::bail!("元テクスチャが見つかりません: {}", source.display());
        }

        let preview_path = Paths::preview_path(model_name);

        let preview_dir = preview_path
            .parent()
            .context("プレビューディレクトリの取得に失敗しました")?;

        std::fs::create_dir_all(preview_dir).with_context(|| {
            format!(
                "プレビューディレクトリの作成に失敗しました: {}",
                preview_dir.display()
            )
        })?;

        // Load source image
        let img = ImageReader::open(source)
            .with_context(|| format!("画像の読み込みに失敗しました: {}", source.display()))?
            .decode()
            .context("画像のデコードに失敗しました")?;

        // Resize to 256x256 with nearest-neighbor (pixel-perfect for Minecraft textures)
        let resized = image::imageops::resize(
            &img,
            256,
            256,
            image::imageops::FilterType::Nearest, // Pixel-perfect scaling
        );

        // Save as PNG
        resized
            .save_with_format(&preview_path, ImageFormat::Png)
            .with_context(|| {
                format!("プレビューの保存に失敗しました: {}", preview_path.display())
            })?;

        println!("✓ プレビューを生成しました: {}", preview_path.display());

        Ok(preview_path)
    }
}
