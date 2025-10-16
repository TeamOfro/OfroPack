use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

/// Parsed issue data for custom model request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIssue {
    pub materials: Vec<String>,
    pub custom_model_data: String,
    pub image_url: String,
}

pub struct IssueParser;

impl IssueParser {
    /// Parse issue body in Markdown format
    pub fn parse(body: &str) -> Result<ParsedIssue> {
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

        Ok(ParsedIssue {
            materials,
            custom_model_data,
            image_url,
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
    pub fn output_github_actions(parsed: &ParsedIssue) -> String {
        format!(
            "materials={}\ncustom_model_data={}\nimage_url={}",
            parsed.materials.join(","),
            parsed.custom_model_data,
            parsed.image_url
        )
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
        assert_eq!(result.materials, vec!["diamond_axe", "golden_axe"]);
        assert_eq!(result.custom_model_data, "test_model");
        assert_eq!(result.image_url, "https://example.com/image.png");
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
}
