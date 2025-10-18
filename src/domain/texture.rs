use anyhow::{Context, Result};
use std::path::Path;

use crate::constants::Paths;
use crate::image_validator::validate_image;
use crate::infra::FileSystem;
use crate::models::AnimationInfo;

/// Handles texture file operations
pub struct TextureHandler;

impl TextureHandler {
    /// Copy and validate texture file
    pub fn install(
        image_path: &Path,
        custom_model_data: &str,
        animation: Option<&AnimationInfo>,
    ) -> Result<()> {
        let texture_path = Paths::texture_path(custom_model_data);

        // Validate texture doesn't already exist
        if texture_path.exists() {
            eprintln!(
                "⚠️  警告: テクスチャファイルが既に存在します: {}\n\
                上書きされます。",
                texture_path.display()
            );
        }

        // Validate image
        println!("🔍 画像を検証中...");
        let allow_animation = animation.is_some();
        let image_info = validate_image(image_path, allow_animation)?;
        println!("  ✓ 画像サイズ: {}", image_info.size_string());

        // Copy texture
        FileSystem::copy(image_path, &texture_path).context(format!(
            "テクスチャをコピーできませんでした: {}",
            texture_path.display()
        ))?;
        println!("  ✓ テクスチャ: {}", texture_path.display());

        Ok(())
    }

    /// Check if texture exists
    pub fn exists(custom_model_data: &str) -> bool {
        let texture_path = Paths::texture_path(custom_model_data);
        FileSystem::exists(&texture_path)
    }

    /// Get texture path
    pub fn path(custom_model_data: &str) -> std::path::PathBuf {
        Paths::texture_path(custom_model_data)
    }
}
