use anyhow::{bail, Context, Result};

use crate::{
    constants::{should_snake_case, IssueType},
    schema::animation::{AnimationData, AnimationInfo},
};

#[derive(Debug)]
pub enum ParsedIssue {
    AddCustomModel {
        materials: Vec<String>,
        custom_model_data: String,
        image_url: String,
        animation: Option<AnimationInfo>,
    },
    Add3DCustomModel {
        materials: Vec<String>,
        custom_model_data: String,
        model_json: String,
        layere_image_urls: Vec<String>,
    },
    Extend {
        materials: Vec<String>,
        custom_model_data: String,
    },
}

pub struct IssueParser;

impl IssueParser {
    pub fn parse(body: &str, issue_type: IssueType) -> Result<ParsedIssue> {
        match issue_type {
            IssueType::Add => Self::parse_add(body),
            IssueType::Extend => Self::parse_extend(body),
        }
    }

    fn parse_add(body: &str) -> Result<ParsedIssue> {
        let materials = Self::extract_field(body, "マテリアル")
            .context("マテリアルフィールドが見つかりません")?;

        if materials == "_No response_" || materials.is_empty() {
            bail!("マテリアルは必須項目です");
        }

        let materials: Vec<String> = materials
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if materials.is_empty() {
            bail!("少なくとも1つのマテリアルを指定してください");
        }

        let custom_model_data = Self::extract_field(body, "カスタムモデルデータ名")
            .context("カスタムモデルデータ名フィールドが見つかりません")?;

        if custom_model_data == "_No response_" || custom_model_data.is_empty() {
            bail!("カスタムモデルデータ名は必須項目です");
        }

        should_snake_case(&custom_model_data)?;

        let image_url =
            Self::extract_field(body, "画像URL").context("画像URLフィールドが見つかりません")?;

        if image_url == "_No response_" || image_url.is_empty() {
            bail!("画像URLは必須項目です");
        }

        if !image_url.starts_with("http://") && !image_url.starts_with("https://") {
            bail!("画像URLはhttp://またはhttps://で始まる必要があります");
        }

        let animation = Self::extract_field(body, "Frametime（アニメーション用・任意）")
            .and_then(|s| {
                if s == "_No response_" || s.is_empty() {
                    None
                } else {
                    s.parse::<u32>().ok().map(|frametime| AnimationInfo {
                        animation: AnimationData { frametime },
                    })
                }
            });

        Ok(ParsedIssue::AddCustomModel {
            materials,
            custom_model_data,
            image_url,
            animation,
        })
    }

    fn parse_extend(body: &str) -> Result<ParsedIssue> {
        let materials = Self::extract_field(body, "マテリアル")
            .context("マテリアルフィールドが見つかりません")?;

        if materials == "_No response_" || materials.is_empty() {
            bail!("マテリアルは必須項目です");
        }

        let materials: Vec<String> = materials
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if materials.is_empty() {
            bail!("少なくとも1つのマテリアルを指定してください");
        }

        let custom_model_data = Self::extract_field(body, "カスタムモデルデータ名")
            .context("カスタムモデルデータ名フィールドが見つかりません")?;

        if custom_model_data == "_No response_" || custom_model_data.is_empty() {
            bail!("カスタムモデルデータ名は必須項目です");
        }

        should_snake_case(&custom_model_data)?;

        Ok(ParsedIssue::Extend {
            materials,
            custom_model_data,
        })
    }

    fn extract_field(body: &str, field_name: &str) -> Option<String> {
        let pattern = format!(
            r"###\s*{}\s*\n\s*(.+?)(?:\n\n###|\n\s*\n|$)",
            regex::escape(field_name)
        );
        let re = regex::Regex::new(&pattern).ok()?;

        re.captures(body)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
    }
}