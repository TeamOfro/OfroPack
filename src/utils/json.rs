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
    std::fs::write(path, json).context("JSONファイルの書き込みに失敗しました")?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let data = std::fs::read_to_string(path).context("JSONファイルの読み込みに失敗しました")?;
    let value = serde_json::from_str(&data).context("JSONのデシリアライズに失敗しました")?;
    Ok(value)
}
