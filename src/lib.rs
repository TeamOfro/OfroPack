//! OfroPack - Minecraft リソースパック管理ツール
//!
//! このクレートは、Minecraftのカスタムモデルデータを管理するための
//! CLIツールを提供します。GitHub Issueからの自動化されたワークフローを
//! サポートし、リソースパックの生成、検証、デプロイを行います。
//!
//! # 主要モジュール
//!
//! - [`cmd`] - CLIコマンドの実装
//! - [`pipeline`] - GitHub連携とCI/CDパイプライン
//! - [`schema`] - Minecraftリソースパックのスキーマ定義
//! - [`utils`] - 共通ユーティリティ関数

pub mod cmd;
pub mod config;
pub mod paths;
pub mod pipeline;
pub mod schema;
pub mod types;
pub mod utils;
pub mod validation;
