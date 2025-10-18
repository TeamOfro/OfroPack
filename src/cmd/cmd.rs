#![allow(clippy::module_inception)]

use crate::constants::{REPO_NAME, REPO_OWNER};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// CLI for OfroPack - Minecraft Resource Pack Manager
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub enum Cmd {
    Add(Add),
    Extend(Extend),
    GenerateGallery(GenerateGallery),
    GenerateMetadata(GenerateMetadata),
    GenerateZip(GenerateZip),
    Runner(Runner),
}

use super::metadata::GenerateMetadata;
use super::zip::GenerateZip;

/// 新しいカスタムモデルをテクスチャと共に追加
#[derive(Parser, Debug)]
pub struct Add {
    /// カンマ区切りのマテリアルリスト (例: diamond_axe,iron_sword)
    #[arg(short, long, value_delimiter = ',', required = true)]
    pub materials: Vec<String>,

    /// カスタムモデルデータ名 (省略時は画像ファイル名を使用)
    #[arg(short, long)]
    pub custom_model_data: Option<String>,

    /// アニメーションのフレームタイム (tick数、アニメーションテクスチャの場合のみ指定)
    #[arg(short = 'f', long)]
    pub frametime: Option<u32>,

    /// テクスチャ画像ファイルのパス
    pub path_to_image: PathBuf,
}

/// 既存のカスタムモデルにマテリアルを追加
#[derive(Parser, Debug)]
pub struct Extend {
    /// カンマ区切りのマテリアルリスト
    #[arg(short, long, value_delimiter = ',', required = true)]
    pub materials: Vec<String>,

    /// カスタムモデルデータ名
    #[arg(short, long, required = true)]
    pub custom_model_data: String,
}

/// ギャラリー用のmodels.jsonを生成
#[derive(Parser, Debug)]
pub struct GenerateGallery {
    /// 出力ファイルパス (デフォルト: models.json)
    #[arg(short, long, default_value = "models.json")]
    pub output: PathBuf,
}

/// GitHub Actions runner utilities
#[derive(Parser, Debug)]
pub struct Runner {
    #[command(subcommand)]
    pub subcommand: RunnerSubcommands,
}

/// GitHub Actions runner utilities
#[derive(Subcommand, Debug)]
pub enum RunnerSubcommands {
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

    /// 成功コメントを投稿 (Add用)
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

    /// 成功コメントを投稿 (Extend用、プレビュー無し)
    PostExtendSuccess {
        /// Issue番号
        #[arg(long)]
        issue_number: u64,

        /// Pull Request番号
        #[arg(long)]
        pr_number: u64,
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
        #[arg(long, default_value = REPO_OWNER)]
        repo_owner: String,

        /// Repository name (for URL generation)
        #[arg(long, default_value = REPO_NAME)]
        repo_name: String,

        /// Branch name (for URL generation)
        #[arg(long, default_value = "main")]
        branch: String,
    },
}
