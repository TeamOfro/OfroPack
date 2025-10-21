use anyhow::{Context, Result, bail};

use crate::{
    constants::{IssueType, ItemModelParent, should_snake_case},
    schema::animation::{AnimationData, AnimationInfo},
};

#[derive(Debug)]
pub enum ParsedIssue {
    Model {
        materials: Vec<String>,
        custom_model_data: String,
        image_url: String,
        animation: Option<AnimationInfo>,
        parent: ItemModelParent,
    },
    Model3d {
        materials: Vec<String>,
        custom_model_data: String,
        model_json_url: String,
        layer_image_urls: Vec<String>,
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
            IssueType::Model => Self::parse_model(body),
            IssueType::Model3d => Self::parse_model3d(body),
            IssueType::Extend => Self::parse_extend(body),
        }
    }

    fn parse_model(body: &str) -> Result<ParsedIssue> {
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

        let parent = Self::extract_field(body, "モデル親")
            .and_then(|s| match s.as_str() {
                "_No response_" | "" => None,
                _ => <ItemModelParent as std::str::FromStr>::from_str(&s).ok(),
            })
            .unwrap_or(ItemModelParent::Handheld);

        let animation =
            Self::extract_field(body, "Frametime（アニメーション用・任意）").and_then(|s| {
                if s == "_No response_" || s.is_empty() {
                    None
                } else {
                    s.parse::<u32>().ok().map(|frametime| AnimationInfo {
                        animation: AnimationData { frametime },
                    })
                }
            });

        Ok(ParsedIssue::Model {
            materials,
            custom_model_data,
            image_url,
            animation,
            parent,
        })
    }

    fn parse_model3d(body: &str) -> Result<ParsedIssue> {
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

        let model_json_url = Self::extract_field(body, "モデルJSONのURL")
            .context("モデルJSONのURLフィールドが見つかりません")?;
        if model_json_url == "_No response_" || model_json_url.is_empty() {
            bail!("モデルJSONのURLは必須項目です");
        }

        let layer_image_urls = Self::extract_field(body, "レイヤー画像のURLリスト")
            .context("レイヤー画像のURLリールドフィールドが見つかりません")?;
        if layer_image_urls == "_No response_" || layer_image_urls.is_empty() {
            bail!("レイヤー画像のURLは必須項目です");
        }
        let layer_image_urls: Vec<String> = layer_image_urls
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if layer_image_urls.is_empty() {
            bail!("少なくとも1つのレイヤー画像のURLを指定してください");
        }

        Ok(ParsedIssue::Model3d {
            materials,
            custom_model_data,
            model_json_url,
            layer_image_urls,
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
