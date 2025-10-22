//! ファイルパス管理
//!
//! リソースパック内のファイルパスを管理する構造体と定数を提供します。

use std::path::PathBuf;

/// リソースパック内のファイルパスを管理する構造体
///
/// この構造体は、Minecraftリソースパック内の各種ファイルへのパスを
/// 一元管理します。テスト時には `TEST_ROOT` 環境変数で
/// ルートディレクトリを上書きできます。
pub struct Paths;

impl Paths {
    fn root() -> PathBuf {
        #[cfg(test)]
        {
            // In tests, use TEST_ROOT environment variable if set
            if let Ok(test_root) = std::env::var("TEST_ROOT") {
                return PathBuf::from(test_root);
            }
        }
        PathBuf::new()
    }

    /// assets ディレクトリパス
    pub const ASSETS: &str = "assets";
    /// アイテム定義ディレクトリパス
    pub const ITEMS: &str = "assets/minecraft/items";
    /// モデルディレクトリパス
    pub const MODELS: &str = "assets/minecraft/models/item";
    /// テクスチャディレクトリパス
    pub const TEXTURES: &str = "assets/minecraft/textures/item";
    /// プレビュー画像ディレクトリパス
    pub const PREVIEWS: &str = "preview";

    /// アイテムとテクスチャのマッピングファイル
    pub const ITEMS_TEXTURES: &str = "items_textures.json";

    /// assets ディレクトリの絶対パスを取得
    pub fn assets_path() -> PathBuf {
        Self::root().join(Self::ASSETS)
    }

    /// 指定したマテリアルのアイテム定義ファイルパスを取得
    pub fn item_path(material: &str) -> PathBuf {
        Self::root().join(format!("{}/{material}.json", Self::ITEMS))
    }

    /// 指定したカスタムモデルデータのモデルファイルパスを取得
    pub fn model_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{custom_model_data}.json", Self::MODELS))
    }

    /// 指定したカスタムモデルデータのテクスチャファイルパスを取得
    pub fn texture_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{custom_model_data}.png", Self::TEXTURES))
    }

    /// 指定したカスタムモデルデータのテクスチャディレクトリパスを取得
    pub fn texture_path_dir(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{custom_model_data}", Self::TEXTURES))
    }

    /// 3Dモデルの指定レイヤーのテクスチャファイルパスを取得
    pub fn texture_layer_path(custom_model_data: &str, layer: usize) -> PathBuf {
        Self::root().join(format!(
            "{}/{custom_model_data}/{layer}.png",
            Self::TEXTURES
        ))
    }

    /// アニメーションメタデータファイルのパスを取得
    pub fn animation_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{custom_model_data}.png.mcmeta", Self::TEXTURES))
    }

    /// プレビュー画像のパスを取得
    pub fn preview_path(custom_model_data: &str) -> PathBuf {
        Self::root().join(format!("{}/{custom_model_data}.png", Self::PREVIEWS))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_path() {
        let path = Paths::item_path("diamond_sword");
        assert!(path.to_string_lossy().contains("diamond_sword.json"));
        assert!(path.to_string_lossy().contains("items"));
    }

    #[test]
    fn test_model_path() {
        let path = Paths::model_path("my_model");
        assert!(path.to_string_lossy().contains("my_model.json"));
        assert!(path.to_string_lossy().contains("models"));
    }

    #[test]
    fn test_texture_path() {
        let path = Paths::texture_path("my_texture");
        assert!(path.to_string_lossy().contains("my_texture.png"));
        assert!(path.to_string_lossy().contains("textures"));
    }

    #[test]
    fn test_texture_layer_path() {
        let path = Paths::texture_layer_path("my_model", 0);
        assert!(path.to_string_lossy().contains("my_model"));
        assert!(path.to_string_lossy().contains("0.png"));
    }
}
