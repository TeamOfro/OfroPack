use std::num::NonZeroU32;
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn create_parent_dir_all(path: &Path) -> anyhow::Result<()> {
    let parent = path.parent().context("Failed to get parent directory")?;
    if !parent.exists() {
        std::fs::create_dir_all(parent).context("Failed to create parent directories")?;
    }
    Ok(())
}

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(value).context("Failed to serialize to JSON")?;
    std::fs::write(path, json_string).context("Failed to write to file")
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> anyhow::Result<T> {
    let file_content = std::fs::read_to_string(path).context("Failed to read file")?;
    serde_json::from_str(&file_content).context("Failed to parse JSON")
}

/// Create .mcmeta file for animation texture
///
/// # Arguments
/// * `texture_path` - Path to the texture file (e.g., "texture.png")
/// * `frametime` - Number of ticks per frame (must be > 0)
///
/// # Returns
/// Path to the created .mcmeta file
///
/// The .mcmeta file will be created next to the texture with ".png.mcmeta" extension
pub fn create_mcmeta_file(texture_path: &Path, frametime: NonZeroU32) -> anyhow::Result<std::path::PathBuf> {
    let mcmeta_path = texture_path.with_extension("png.mcmeta");

    let mcmeta_content = json!({
        "animation": {
            "frametime": frametime.get()
        }
    });

    let json_string = serde_json::to_string_pretty(&mcmeta_content)
        .context("Failed to serialize .mcmeta JSON")?;

    std::fs::write(&mcmeta_path, json_string)
        .context(format!("Failed to write .mcmeta file: {}", mcmeta_path.display()))?;

    Ok(mcmeta_path)
}
