use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::models::AnimationInfo;

/// Parsed issue data for custom model request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedIssueData {
    Add {
        materials: Vec<String>,
        custom_model_data: String,
        image_url: String,
        animation: Option<AnimationInfo>,
    },
    Extend {
        materials: Vec<String>,
        custom_model_data: String,
    },
}

pub struct IssueParser;

impl IssueParser {
    /// Parse issue body based on labels to detect issue type
    #[allow(dead_code)] // For future use with workflow label-based routing
    pub fn parse_with_labels(body: &str, labels: &[String]) -> Result<ParsedIssueData> {
        if labels.contains(&"extend-model".to_string()) {
            Self::parse_extend(body)
        } else if labels.contains(&"custom-model".to_string()) {
            Self::parse_add(body)
        } else {
            bail!(
                "このIssueは対応していないラベルです。'custom-model' または 'extend-model' ラベルが必要です。"
            );
        }
    }

    /// Parse issue body in Markdown format (legacy, assumes Add type)
    pub fn parse(body: &str) -> Result<ParsedIssueData> {
        Self::parse_add(body)
    }

    /// Parse Add type issue
    fn parse_add(body: &str) -> Result<ParsedIssueData> {
        // Parse materials (required)
        let materials = Self::extract_field(body, "Materials")
            .or_else(|| Self::extract_field(body, "マテリアル"))
            .context("マテリアルフィールドが見つかりません")?;

        if materials == "_No response_" || materials.is_empty() {
            bail!("マテリアルは必須項目です");
        }

        // Parse as comma-separated list
        let materials: Vec<String> = materials
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if materials.is_empty() {
            bail!("少なくとも1つのマテリアルを指定してください");
        }

        // Parse custom model data (required)
        let custom_model_data = Self::extract_field(body, "Custom Model Data")
            .or_else(|| Self::extract_field(body, "カスタムモデルデータ名"))
            .context("カスタムモデルデータ名フィールドが見つかりません")?;

        if custom_model_data == "_No response_" || custom_model_data.is_empty() {
            bail!("カスタムモデルデータ名は必須項目です");
        }

        // Validate custom_model_data name (alphanumeric, underscore, hyphen only)
        if !custom_model_data
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            bail!("カスタムモデルデータ名は英数字、アンダースコア、ハイフンのみ使用できます");
        }

        // Parse image URL (required)
        let image_url = Self::extract_field(body, "Image URL")
            .or_else(|| Self::extract_field(body, "画像URL"))
            .context("画像URLフィールドが見つかりません")?;

        if image_url == "_No response_" || image_url.is_empty() {
            bail!("画像URLは必須項目です");
        }

        // Validate URL format
        if !image_url.starts_with("http://") && !image_url.starts_with("https://") {
            bail!("画像URLはhttp://またはhttps://で始まる必要があります");
        }

        // Parse frametime (optional)
        let animation = Self::extract_field(body, "Frametime")
            .or_else(|| Self::extract_field(body, "Frametime（アニメーション用・任意）"))
            .and_then(|s| {
                if s == "_No response_" || s.is_empty() {
                    None
                } else {
                    match s.parse::<u32>() {
                        Ok(0) => {
                            // Zero is invalid, ignore it
                            None
                        }
                        Ok(n) => std::num::NonZeroU32::new(n).map(AnimationInfo::new),
                        Err(_) => None,
                    }
                }
            });

        Ok(ParsedIssueData::Add {
            materials,
            custom_model_data,
            image_url,
            animation,
        })
    }

    /// Parse Extend type issue
    fn parse_extend(body: &str) -> Result<ParsedIssueData> {
        // Parse materials (required)
        let materials = Self::extract_field(body, "Materials")
            .or_else(|| Self::extract_field(body, "マテリアル"))
            .context("マテリアルフィールドが見つかりません")?;

        if materials == "_No response_" || materials.is_empty() {
            bail!("マテリアルは必須項目です");
        }

        // Parse as comma-separated list
        let materials: Vec<String> = materials
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if materials.is_empty() {
            bail!("少なくとも1つのマテリアルを指定してください");
        }

        // Parse custom model data (required)
        let custom_model_data = Self::extract_field(body, "Custom Model Data")
            .or_else(|| Self::extract_field(body, "カスタムモデルデータ名"))
            .context("カスタムモデルデータ名フィールドが見つかりません")?;

        if custom_model_data == "_No response_" || custom_model_data.is_empty() {
            bail!("カスタムモデルデータ名は必須項目です");
        }

        // Validate custom_model_data name (alphanumeric, underscore, hyphen only)
        if !custom_model_data
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            bail!("カスタムモデルデータ名は英数字、アンダースコア、ハイフンのみ使用できます");
        }

        Ok(ParsedIssueData::Extend {
            materials,
            custom_model_data,
        })
    }

    /// Extract field value from issue body
    /// Format: ### Field Name\n\nvalue
    fn extract_field(body: &str, field_name: &str) -> Option<String> {
        // Match pattern: ### Field Name\n\nvalue
        let pattern = format!(
            r"###\s*{}\s*\n\s*(.+?)(?:\n\n###|\n\s*\n|$)",
            regex::escape(field_name)
        );
        let re = regex::Regex::new(&pattern).ok()?;

        re.captures(body)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
    }

    /// Output in GitHub Actions format (key=value)
    pub fn output_github_actions(parsed: &ParsedIssueData) -> String {
        match parsed {
            ParsedIssueData::Add {
                materials,
                custom_model_data,
                image_url,
                animation,
            } => {
                let frametime_part = animation
                    .as_ref()
                    .map(|anim| format!("\nframetime={}", anim.frametime.get()))
                    .unwrap_or_default();

                format!(
                    "issue_type=add\nmaterials={}\ncustom_model_data={}\nimage_url={}{}",
                    materials.join(","),
                    custom_model_data,
                    image_url,
                    frametime_part
                )
            }
            ParsedIssueData::Extend {
                materials,
                custom_model_data,
            } => {
                format!(
                    "issue_type=extend\nmaterials={}\ncustom_model_data={}",
                    materials.join(","),
                    custom_model_data
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_issue() {
        let body = r#"
### Materials

diamond_axe, golden_axe

### Custom Model Data

test_model

### Image URL

https://example.com/image.png

### Additional Notes

Test note
"#;

        let result = IssueParser::parse(body).unwrap();
        match result {
            ParsedIssueData::Add {
                materials,
                custom_model_data,
                image_url,
                animation,
            } => {
                assert_eq!(materials, vec!["diamond_axe", "golden_axe"]);
                assert_eq!(custom_model_data, "test_model");
                assert_eq!(image_url, "https://example.com/image.png");
                assert!(animation.is_none());
            }
            ParsedIssueData::Extend { .. } => panic!("Expected Add variant"),
        }
    }

    #[test]
    fn test_parse_with_frametime() {
        let body = r#"
### Materials

diamond_axe

### Custom Model Data

animated_model

### Image URL

https://example.com/animated.png

### Frametime（アニメーション用・任意）

5

"#;

        let result = IssueParser::parse(body).unwrap();
        match result {
            ParsedIssueData::Add { animation, .. } => {
                assert_eq!(animation.map(|anim| anim.frametime.get()), Some(5));
            }
            ParsedIssueData::Extend { .. } => panic!("Expected Add variant"),
        }
    }

    #[test]
    fn test_parse_missing_materials() {
        let body = r#"
### Custom Model Data

test_model

### Image URL

https://example.com/image.png
"#;

        assert!(IssueParser::parse(body).is_err());
    }

    #[test]
    fn test_parse_invalid_custom_model_data() {
        let body = r#"
### Materials

diamond_axe

### Custom Model Data

test model!@#

### Image URL

https://example.com/image.png
"#;

        assert!(IssueParser::parse(body).is_err());
    }

    #[test]
    fn test_parse_extend_issue() {
        let body = r#"
### Materials

diamond_axe, golden_axe

### Custom Model Data

test_model

"#;

        let result = IssueParser::parse_extend(body).unwrap();
        match result {
            ParsedIssueData::Extend {
                materials,
                custom_model_data,
            } => {
                assert_eq!(materials, vec!["diamond_axe", "golden_axe"]);
                assert_eq!(custom_model_data, "test_model");
            }
            ParsedIssueData::Add { .. } => panic!("Expected Extend variant"),
        }
    }
}
