use std::path::Path;

use anyhow::{Context, Result};

/// Validate image file
pub fn validate_image(path: &Path) -> Result<ImageInfo> {
    // Check if file exists
    if !path.exists() {
        anyhow::bail!("画像ファイルが存在しません: {}", path.display());
    }

    // Try to load image
    let img = image::open(path)
        .context(format!("画像の読み込みに失敗しました: {}", path.display()))?;

    let width = img.width();
    let height = img.height();

    // Check if dimensions are power of 2
    if !is_power_of_two(width) || !is_power_of_two(height) {
        anyhow::bail!(
            "画像の縦横サイズが2の累乗ではありません\n\
            ファイル: {}\n\
            現在のサイズ: {}x{}\n\n\
            ⚠️  警告: Minecraftのテクスチャは通常 16x16, 32x32, 64x64 などの2の累乗サイズである必要があります\n\
            💡 推奨: 画像を 16x16, 32x32, 64x64, 128x128 などにリサイズしてください",
            path.display(),
            width,
            height
        );
    }

    if width != height {
        anyhow::bail!(
            "画像が正方形ではありません\n\
            ファイル: {}\n\
            現在のサイズ: {}x{}\n\n\
            ⚠️  警告: Minecraftのテクスチャは通常正方形です\n\
            💡 推奨: 画像を正方形 (例: 16x16, 32x32, 64x64, 128x128) にリサイズしてください",
            path.display(),
            width,
            height
        );
    }

    Ok(ImageInfo { width, height })
}

/// Check if a number is a power of 2
fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
}

impl ImageInfo {
    pub fn size_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(16));
        assert!(is_power_of_two(32));
        assert!(is_power_of_two(64));
        assert!(is_power_of_two(128));
        assert!(is_power_of_two(256));
        assert!(is_power_of_two(512));
        assert!(is_power_of_two(1024));

        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(7));
        assert!(!is_power_of_two(9));
        assert!(!is_power_of_two(15));
        assert!(!is_power_of_two(17));
        assert!(!is_power_of_two(100));
    }
}
