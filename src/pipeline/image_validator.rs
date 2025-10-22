//! 画像ファイルの検証
//!
//! PNG画像の形式とサイズを検証する機能を提供します。

use std::path::{Path, PathBuf};

use anyhow::Context;
use image::GenericImageView;

use crate::schema::animation::AnimationInfo;

/// 画像ファイルのバリデーター
///
/// PNG形式の画像ファイルを検証し、Minecraftリソースパックの
/// 要件を満たしているかチェックします。
pub struct ImageValidator {
    path: PathBuf,
    dimensions: (u32, u32),
}

impl ImageValidator {
    /// PNG画像ファイルを開いて検証する
    ///
    /// # Errors
    ///
    /// - 画像ファイルが読み込めない場合
    /// - PNG形式でない場合
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use processor::pipeline::image_validator::ImageValidator;
    /// use std::path::Path;
    ///
    /// let validator = ImageValidator::new_png(Path::new("texture.png"))?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new_png(path: &Path) -> anyhow::Result<Self> {
        let img = image::open(path)
            .with_context(|| format!("画像ファイルの読み込みに失敗しました: {}", path.display()))?;

        // Check if it's PNG format by trying to load it
        // The image crate will error if it's not a valid image format
        let format =
            image::guess_format(&std::fs::read(path)?).context("画像形式を判定できませんでした")?;

        if format != image::ImageFormat::Png {
            anyhow::bail!("PNG形式の画像のみ対応しています（検出された形式: {format:?}）");
        }

        let (width, height) = img.dimensions();
        println!("画像サイズ: {width}x{height}");
        Ok(Self {
            path: path.to_path_buf(),
            dimensions: (width, height),
        })
    }

    /// 画像ファイルのパスを取得
    #[must_use]
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }

    /// 2Dモデル用の画像として妥当かチェック
    ///
    /// # Errors
    ///
    /// - 画像の幅または高さが2の累乗でない場合
    /// - アニメーションの場合、高さが幅の倍数でない場合
    /// - 静止画の場合、正方形でない場合
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
                "アニメーションフレーム数: {frame_count}, 指定されたフレームタイム: {}",
                animation_info.animation.frametime
            );
        } else if width != height {
            anyhow::bail!("静止画の場合、画像は正方形である必要があります");
        }

        Ok(())
    }
}

/// 数値が2の累乗かどうかを判定
fn is_pow_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pow_of_two() {
        assert!(is_pow_of_two(1));
        assert!(is_pow_of_two(2));
        assert!(is_pow_of_two(4));
        assert!(is_pow_of_two(8));
        assert!(is_pow_of_two(16));
        assert!(is_pow_of_two(32));
        assert!(is_pow_of_two(64));
        assert!(is_pow_of_two(128));
        assert!(is_pow_of_two(256));

        assert!(!is_pow_of_two(0));
        assert!(!is_pow_of_two(3));
        assert!(!is_pow_of_two(5));
        assert!(!is_pow_of_two(15));
        assert!(!is_pow_of_two(100));
    }

    #[test]
    fn test_validate_png_with_valid_png() {
        use image::{ImageBuffer, Rgb};
        use tempfile::Builder;

        // Create a 16x16 PNG using image crate
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(16, 16, |_x, _y| {
            Rgb([255, 0, 0]) // Red pixel
        });

        // Create temp file with .png extension
        let temp_file = Builder::new().suffix(".png").tempfile().unwrap();
        let temp_path = temp_file.path().to_owned();

        // Save the image
        img.save_with_format(&temp_path, image::ImageFormat::Png)
            .unwrap();

        // Test that we can create a validator (don't print in tests for cleaner output)
        let validator = ImageValidator::new_png(&temp_path);
        assert!(
            validator.is_ok(),
            "Should successfully validate PNG: {:?}",
            validator.err()
        );

        let validator = validator.unwrap();
        assert!(
            validator.should_model(None).is_ok(),
            "16x16 should be valid as static texture"
        );

        // temp_file will be dropped and deleted here
    }
}
