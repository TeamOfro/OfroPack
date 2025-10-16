use anyhow::{Context, Result};
use image::{ImageFormat, ImageReader};
use std::path::{Path, PathBuf};

pub struct PreviewGenerator {
    preview_dir: PathBuf,
}

impl PreviewGenerator {
    pub fn new(preview_dir: impl Into<PathBuf>) -> Self {
        Self {
            preview_dir: preview_dir.into(),
        }
    }

    /// Generate 256x256 preview from source texture with nearest-neighbor interpolation
    pub fn generate(&self, source: &Path, model_name: &str) -> Result<PathBuf> {
        if !source.exists() {
            anyhow::bail!("Source texture not found: {}", source.display());
        }

        // Create preview directory
        std::fs::create_dir_all(&self.preview_dir).context("Failed to create preview directory")?;

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

        // Output path
        let preview_path = self.preview_dir.join(format!("{}.png", model_name));

        // Save as PNG
        resized
            .save_with_format(&preview_path, ImageFormat::Png)
            .with_context(|| format!("Failed to save preview: {}", preview_path.display()))?;

        println!("✓ Preview generated: {}", preview_path.display());

        Ok(preview_path)
    }

    /// Generate preview URL for GitHub raw content
    pub fn generate_url(
        repo_owner: &str,
        repo_name: &str,
        branch: &str,
        preview_path: &Path,
    ) -> String {
        // Convert to relative path if absolute
        let path_str = if preview_path.is_absolute() {
            preview_path
                .file_name()
                .and_then(|f| f.to_str())
                .map(|f| format!("preview/{}", f))
                .unwrap_or_else(|| preview_path.display().to_string())
        } else {
            preview_path.display().to_string()
        };

        format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            repo_owner, repo_name, branch, path_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_url() {
        let path = Path::new("preview/test_model.png");
        let url = PreviewGenerator::generate_url("owner", "repo", "main", path);
        assert_eq!(
            url,
            "https://raw.githubusercontent.com/owner/repo/main/preview/test_model.png"
        );
    }
}
