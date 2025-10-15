use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

/// CLI for OfroPack - Minecraft Resource Pack Manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 新しいカスタムモデルをテクスチャと共に追加
    Add {
        /// カンマ区切りのマテリアルリスト (例: diamond_axe,iron_sword)
        #[arg(short, long, value_delimiter = ',', required = true)]
        materials: Vec<String>,

        /// カスタムモデルデータ名 (省略時は画像ファイル名を使用)
        #[arg(short, long)]
        custom_model_data: Option<String>,

        /// テクスチャ画像ファイルのパス
        path_to_image: PathBuf,
    },

    /// 既存のカスタムモデルにマテリアルを追加
    Extend {
        /// カンマ区切りのマテリアルリスト
        #[arg(short, long, value_delimiter = ',', required = true)]
        materials: Vec<String>,

        /// カスタムモデルデータ名
        #[arg(short, long, required = true)]
        custom_model_data: String,
    },

    /// ギャラリー用のmodels.jsonを生成
    GenerateGallery {
        /// 出力ファイルパス (デフォルト: models.json)
        #[arg(short, long, default_value = "models.json")]
        output: PathBuf,
    },
}

impl Cli {
    pub fn validate(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Add {
                path_to_image,
                materials,
                ..
            } => {
                if materials.is_empty() {
                    return Err(anyhow::anyhow!(
                        "少なくとも1つのマテリアルを指定してください\n\
                        例: -m diamond_axe,iron_sword"
                    ));
                }

                if !path_to_image.exists() {
                    return Err(anyhow::anyhow!(
                        "画像ファイルが見つかりません: {}",
                        path_to_image.display()
                    ));
                }
            }
            Commands::Extend {
                materials,
                custom_model_data,
            } => {
                if materials.is_empty() {
                    return Err(anyhow::anyhow!(
                        "少なくとも1つのマテリアルを指定してください"
                    ));
                }

                if !custom_model_data
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                {
                    return Err(anyhow::anyhow!(
                        "カスタムモデルデータ名は小文字英数字とアンダースコアのみ使用できます"
                    ));
                }
            }
            Commands::GenerateGallery { output: _ } => {
                // No validation needed
            }
        }
        Ok(())
    }
}

pub fn get_custom_model_data(
    custom_model_data: &Option<String>,
    path_to_image: &Path,
) -> anyhow::Result<String> {
    let custom_model_data = match custom_model_data {
        Some(name) => name.to_lowercase(),
        None => path_to_image
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("画像ファイル名を取得できません"))?
            .to_string(),
    };

    if !custom_model_data
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(anyhow::anyhow!(
            "カスタムモデルデータ名は小文字英数字とアンダースコアのみ使用できます"
        ));
    }

    Ok(custom_model_data)
}

pub fn normalize_materials(materials: &[String]) -> Vec<String> {
    materials.iter().map(|m| m.to_lowercase()).collect()
}
