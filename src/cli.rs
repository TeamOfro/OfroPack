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

    /// GitHub Actions runner utilities
    Runner {
        #[command(subcommand)]
        subcommand: RunnerCommands,
    },
}

/// GitHub Actions runner utilities
#[derive(Subcommand, Debug)]
pub enum RunnerCommands {
    /// Issue全体を処理（ダウンロード、検証、モデル追加、プレビュー生成）
    ProcessIssue {
        /// Issue番号
        #[arg(long)]
        issue_number: u64,

        /// IssueのBody（Markdown形式）
        #[arg(long)]
        body: String,
    },

    /// Issue Bodyを解析してGitHub Actions形式で出力
    ParseIssue {
        /// Issue body text (Markdown format)
        #[arg(long)]
        body: String,
    },

    /// 成功コメントを投稿
    PostSuccess {
        /// Issue番号
        #[arg(long)]
        issue_number: u64,

        /// Pull Request番号
        #[arg(long)]
        pr_number: u64,

        /// プレビュー画像URL
        #[arg(long)]
        preview_url: String,
    },

    /// 失敗コメントを投稿してIssueをクローズ
    PostFailure {
        /// Issue番号
        #[arg(long)]
        issue_number: u64,

        /// エラーメッセージ
        #[arg(long)]
        error_message: String,

        /// ワークフローURL
        #[arg(long)]
        workflow_url: String,
    },

    /// Post a comment to an issue
    Comment {
        /// Issue number
        #[arg(long)]
        issue_number: u64,

        /// Comment body (Markdown format)
        #[arg(long)]
        body: String,
    },

    /// Add a reaction to an issue
    React {
        /// Issue number
        #[arg(long)]
        issue_number: u64,

        /// Reaction type (+1, -1, laugh, confused, heart, hooray, rocket, eyes)
        #[arg(long)]
        reaction: String,
    },

    /// Close an issue
    CloseIssue {
        /// Issue number
        #[arg(long)]
        issue_number: u64,
    },

    /// Generate preview image (256x256, pixel-perfect)
    GeneratePreview {
        /// Source texture path
        #[arg(long)]
        source: PathBuf,

        /// Model name
        #[arg(long)]
        model_name: String,

        /// Preview directory (default: preview)
        #[arg(long, default_value = "preview")]
        preview_dir: PathBuf,

        /// Repository owner (for URL generation)
        #[arg(long)]
        repo_owner: Option<String>,

        /// Repository name (for URL generation)
        #[arg(long)]
        repo_name: Option<String>,

        /// Branch name (for URL generation)
        #[arg(long)]
        branch: Option<String>,
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
            Commands::Runner { subcommand } => match subcommand {
                RunnerCommands::ProcessIssue { body, .. } => {
                    if body.is_empty() {
                        return Err(anyhow::anyhow!("Issue bodyは必須です"));
                    }
                }
                RunnerCommands::PostSuccess { preview_url, .. } => {
                    if preview_url.is_empty() {
                        return Err(anyhow::anyhow!("プレビューURLは必須です"));
                    }
                }
                RunnerCommands::PostFailure {
                    error_message,
                    workflow_url,
                    ..
                } => {
                    if error_message.is_empty() || workflow_url.is_empty() {
                        return Err(anyhow::anyhow!(
                            "エラーメッセージとワークフローURLは必須です"
                        ));
                    }
                }
                RunnerCommands::ParseIssue { body } => {
                    if body.is_empty() {
                        return Err(anyhow::anyhow!("Issue bodyは必須です"));
                    }
                }
                RunnerCommands::Comment { body, .. } => {
                    if body.is_empty() {
                        return Err(anyhow::anyhow!("コメント本文は必須です"));
                    }
                }
                RunnerCommands::React { reaction, .. } => {
                    let valid_reactions = [
                        "+1", "-1", "laugh", "confused", "heart", "hooray", "rocket", "eyes",
                    ];
                    if !valid_reactions.contains(&reaction.as_str()) {
                        return Err(anyhow::anyhow!(
                            "無効なReactionです。有効なReaction: {}",
                            valid_reactions.join(", ")
                        ));
                    }
                }
                RunnerCommands::CloseIssue { .. } => {
                    // No validation needed
                }
                RunnerCommands::GeneratePreview { source, .. } => {
                    if !source.exists() {
                        return Err(anyhow::anyhow!(
                            "ソーステクスチャが見つかりません: {}",
                            source.display()
                        ));
                    }
                }
            },
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
