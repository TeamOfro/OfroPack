use anyhow::{Context, Result};
use image::{ImageFormat, ImageReader};
use std::path::{Path, PathBuf};

use crate::constants::Paths;

pub struct PreviewGenerator;

impl PreviewGenerator {
    /// Generate 256x256 preview from source texture with nearest-neighbor interpolation
    pub fn generate(source: &Path, model_name: &str) -> Result<PathBuf> {
        if !source.exists() {
            anyhow::bail!("Source texture not found: {}", source.display());
        }

        let preview_path = Paths::preview_path(model_name);

        let preview_dir = preview_path
            .parent()
            .context("Failed to get preview directory")?;

        std::fs::create_dir_all(preview_dir).with_context(|| {
            format!(
                "Failed to create preview directory: {}",
                preview_dir.display()
            )
        })?;

        // Load source image
        let img = ImageReader::open(source)
            .with_context(|| format!("Failed to open source image: {}", source.display()))?
            .decode()
            .context("Failed to decode image")?;

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
            .with_context(|| format!("Failed to save preview: {}", preview_path.display()))?;

        println!("✓ Preview generated: {}", preview_path.display());

        Ok(preview_path)
    }
}
