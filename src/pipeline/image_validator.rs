use std::path::{Path, PathBuf};

use anyhow::Context;
use image::GenericImageView;

use crate::schema::animation::AnimationInfo;

pub struct ImageValidator {
    path: PathBuf,
    dimensions: (u32, u32),
}

impl ImageValidator {
    pub fn new_png(path: &Path) -> anyhow::Result<Self> {
        let img = image::open(path)
            .with_context(|| format!("画像ファイルの読み込みに失敗しました: {}", path.display()))?;

        // Check if it's PNG format by trying to load it
        // The image crate will error if it's not a valid image format
        let format =
            image::guess_format(&std::fs::read(path)?).context("画像形式を判定できませんでした")?;

        if format != image::ImageFormat::Png {
            anyhow::bail!(
                "PNG形式の画像のみ対応しています（検出された形式: {:?}）",
                format
            );
        }

        let (width, height) = img.dimensions();
        println!("画像サイズ: {}x{}", width, height);
        Ok(Self {
            path: path.to_path_buf(),
            dimensions: (width, height),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn should_model(&self, info: Option<&AnimationInfo>) -> anyhow::Result<()> {
        let (width, height) = self.dimensions;
        if !is_pow_of_two(width) || !is_pow_of_two(height) {
            anyhow::bail!("画像の幅と高さは2の累乗である必要があります (16,32,64)");
        }
        if let Some(animation_info) = info {
            let frame_count = height / width;
            if frame_count == 0 {
                anyhow::bail!("アニメーション画像の高さが幅の倍数ではありません");
            }
            println!(
                "アニメーションフレーム数: {}, 指定されたフレームタイム: {}",
                frame_count, animation_info.animation.frametime
            );
        } else if width != height {
            anyhow::bail!("静止画の場合、画像は正方形である必要があります");
        }

        Ok(())
    }
}

fn is_pow_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
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

        let _ = ImageValidator::new_png(temp_file.path()).unwrap();
    }
}
