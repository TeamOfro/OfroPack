use anyhow::{Context, Result};
use std::path::Path;

use crate::constants::Paths;
use crate::file_utils::create_mcmeta_file;
use crate::image_validator::validate_image;
use crate::models::AnimationInfo;

/// Handles animation-related operations
pub struct AnimationHandler;

impl AnimationHandler {
    /// Validate animation and create .mcmeta file
    pub fn install(
        image_path: &Path,
        custom_model_data: &str,
        animation_info: &AnimationInfo,
    ) -> Result<AnimationInfo> {
        // Validate image to get frame count
        let image_info = validate_image(image_path, true)?;

        // Update animation with frame count (clone to avoid moving)
        let animation = animation_info
            .clone()
            .with_frame_count(image_info.frame_count);

        // Create .mcmeta file
        let texture_path = Paths::texture_path(custom_model_data);
        let mcmeta_path = create_mcmeta_file(&texture_path, animation.frametime)?;

        println!(
            "  ✓ アニメーション設定: {} (frametime: {})",
            mcmeta_path.display(),
            animation.frametime.get()
        );

        Ok(animation)
    }

    /// Get animation metadata from existing .mcmeta file
    pub fn get_metadata(
        custom_model_data: &str,
        textures_dir: &Path,
    ) -> Result<Option<AnimationMetadata>> {
        let mcmeta_path = textures_dir.join(format!("{}.png.mcmeta", custom_model_data));

        if !mcmeta_path.exists() {
            return Ok(None);
        }

        #[derive(serde::Deserialize)]
        struct McMetaFile {
            animation: McMetaAnimation,
        }

        #[derive(serde::Deserialize)]
        struct McMetaAnimation {
            frametime: u32,
        }

        let content = std::fs::read_to_string(&mcmeta_path).context(format!(
            ".mcmetaファイルの読み込みに失敗: {}",
            mcmeta_path.display()
        ))?;

        let mcmeta: McMetaFile = serde_json::from_str(&content).context(format!(
            ".mcmetaファイルのパースに失敗: {}",
            mcmeta_path.display()
        ))?;

        // Get frame count from image dimensions
        let image_path = textures_dir.join(format!("{}.png", custom_model_data));
        let frame_count = if let Ok(img) = image::open(&image_path) {
            use image::GenericImageView;
            let (width, height) = img.dimensions();
            height / width
        } else {
            anyhow::bail!(
                "アニメーション画像の読み込みに失敗: {}",
                image_path.display()
            );
        };

        Ok(Some(AnimationMetadata {
            frame_count,
            frametime: mcmeta.animation.frametime,
        }))
    }
}

/// Animation metadata for gallery
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AnimationMetadata {
    pub frame_count: u32,
    pub frametime: u32,
}
