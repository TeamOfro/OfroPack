use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};

pub fn create_parent_dir_all(path: &Path) -> anyhow::Result<()> {
    let parent = path.parent().context("Failed to get parent directory")?;
    if !parent.exists() {
        std::fs::create_dir_all(parent).context("Failed to create parent directories")?;
    }
    Ok(())
}

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(value).context("Failed to serialize to JSON")?;
    std::fs::write(path, json_string).context("Failed to write to file")
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> anyhow::Result<T> {
    let file_content = std::fs::read_to_string(path).context("Failed to read file")?;
    serde_json::from_str(&file_content).context("Failed to parse JSON")
}
