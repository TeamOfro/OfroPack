use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Git metadata for a file
#[derive(Debug, Clone)]
pub struct GitMetadata {
    pub added_date: String,
    pub author: String,
}

impl GitMetadata {
    /// Get git metadata for a file (when it was added and by whom)
    pub fn for_file(file_path: &Path) -> Result<Self> {
        let output = Command::new("git")
            .args([
                "log",
                "--diff-filter=A",
                "--format=%aI|%an",
                "--",
                file_path.to_str().unwrap_or(""),
            ])
            .output();

        if let Ok(output) = output
            && output.status.success()
        {
            let log = String::from_utf8_lossy(&output.stdout);
            let log = log.trim();
            let log = log.lines().next().unwrap_or(log);

            if !log.is_empty() {
                let parts: Vec<&str> = log.split('|').collect();
                if parts.len() >= 2 {
                    return Ok(Self {
                        added_date: parts[0].to_string(),
                        author: parts[1].to_string(),
                    });
                }
            }
        }

        // Fallback to file modification time
        Self::from_file_mtime(file_path)
    }

    /// Fallback: use file modification time
    fn from_file_mtime(file_path: &Path) -> Result<Self> {
        let metadata = std::fs::metadata(file_path)
            .context(format!("ファイルの情報取得に失敗: {}", file_path.display()))?;

        let modified = metadata
            .modified()
            .context("ファイルの更新日時の取得に失敗")?;

        let datetime: chrono::DateTime<chrono::Utc> = modified.into();

        Ok(Self {
            added_date: datetime.to_rfc3339(),
            author: "Unknown".to_string(),
        })
    }

    /// Fallback to current time
    pub fn unknown() -> Self {
        Self {
            added_date: chrono::Utc::now().to_rfc3339(),
            author: "Unknown".to_string(),
        }
    }
}
