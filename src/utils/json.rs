use std::path::Path;

use anyhow::Context;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn merge_json(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                merge_json(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b.clone(),
    }
}

pub fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(value).context("JSONのシリアライズに失敗しました")?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "JSONの親ディレクトリの作成に失敗しました: {}",
                parent.display()
            )
        })?;
    }
    std::fs::write(path, json)
        .with_context(|| format!("JSONファイルの書き込みに失敗しました: {}", path.display()))?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("JSONファイルの読み込みに失敗しました: {}", path.display()))?;
    let value = serde_json::from_str(&data).context("JSONのデシリアライズに失敗しました")?;
    Ok(value)
}
