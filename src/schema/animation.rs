//! アニメーションテクスチャのメタデータ定義
//!
//! Minecraftのアニメーションテクスチャ用の `.mcmeta` ファイルの
//! スキーマを定義します。

use serde::{Deserialize, Serialize};

/// アニメーション情報（.mcmetaファイルの最上位構造）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationInfo {
    /// アニメーション設定
    pub animation: AnimationData,
}

/// アニメーション設定
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationData {
    /// フレームごとのtick数（1 tick = 1/20秒）
    pub frametime: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_info_serialization() {
        let info = AnimationInfo {
            animation: AnimationData { frametime: 5 },
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("animation"));
        assert!(json.contains("frametime"));
        assert!(json.contains("5"));
    }

    #[test]
    fn test_animation_info_deserialization() {
        let json = r#"{"animation":{"frametime":10}}"#;
        let info: AnimationInfo = serde_json::from_str(json).unwrap();

        assert_eq!(info.animation.frametime, 10);
    }
}
