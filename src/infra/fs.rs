use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Abstraction over file system operations
pub struct FileSystem;

impl FileSystem {
    /// Read JSON file and deserialize
    pub fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
        let content = fs::read_to_string(path)
            .context(format!("ファイルの読み込みに失敗: {}", path.display()))?;
        serde_json::from_str(&content).context(format!("JSONのパースに失敗: {}", path.display()))
    }

    /// Write data as JSON file
    pub fn write_json<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
        Self::create_parent_dirs(path)?;
        let json = serde_json::to_string_pretty(data).context("JSONのシリアライズに失敗")?;
        fs::write(path, json).context(format!("ファイルの書き込みに失敗: {}", path.display()))
    }

    /// Create parent directories if they don't exist
    pub fn create_parent_dirs(path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context(format!("ディレクトリの作成に失敗: {}", parent.display()))?;
        }
        Ok(())
    }

    /// Copy file
    pub fn copy(from: &Path, to: &Path) -> Result<()> {
        Self::create_parent_dirs(to)?;
        fs::copy(from, to).context(format!(
            "ファイルのコピーに失敗: {} -> {}",
            from.display(),
            to.display()
        ))?;
        Ok(())
    }

    /// Write text file
    pub fn write_text(path: &Path, content: &str) -> Result<()> {
        Self::create_parent_dirs(path)?;
        fs::write(path, content).context(format!("ファイルの書き込みに失敗: {}", path.display()))
    }

    /// Check if file exists
    pub fn exists(path: &Path) -> bool {
        path.exists()
    }

    /// List files in directory with extension filter
    pub fn list_files(dir: &Path, extension: Option<&str>) -> Result<Vec<std::path::PathBuf>> {
        if !dir.exists() {
            return Ok(vec![]);
        }

        let entries = fs::read_dir(dir)
            .context(format!("ディレクトリの読み込みに失敗: {}", dir.display()))?;

        let mut files = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if let Some(ext) = extension {
                if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                    files.push(path);
                }
            } else if path.is_file() {
                files.push(path);
            }
        }

        Ok(files)
    }

    /// Read text file
    pub fn read_text(path: &Path) -> Result<String> {
        fs::read_to_string(path).context(format!("ファイルの読み込みに失敗: {}", path.display()))
    }
}
