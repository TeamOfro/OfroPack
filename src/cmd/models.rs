use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

use crate::{
    constants::Paths,
    schema::{animation::AnimationInfo, items::ItemResource},
    utils::json::{read_json, write_json},
};

#[derive(clap::Parser, Debug)]
pub struct Models {
    #[clap(default_value = "models.json")]
    pub output: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsData {
    pub models: Vec<ModelInfo>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub materials: Vec<String>,
    pub texture_path: String,
    pub added_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<AnimationMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationMetadata {
    pub frame_count: u32,
    pub frametime: u32,
}

impl super::Run for Models {
    fn run(&self) -> anyhow::Result<()> {
        let mut material_map_by_model: HashMap<String, Vec<String>> = HashMap::new();

        for entry in std::fs::read_dir(Paths::ITEMS)
            .context("アイテムディレクトリの読み込みに失敗")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
        {
            let material_path = entry.path();
            let material = material_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if material.is_empty() {
                eprintln!("  ✗ 無効なアイテムファイル名: {:?}", material_path);
                continue;
            }
            let item_resource = read_json::<ItemResource>(&material_path).with_context(|| {
                format!(
                    "アイテムリソースの読み込みに失敗: {}",
                    material_path.display()
                )
            })?;
            item_resource
                .model
                .cases
                .iter()
                .map(|case| case.when.clone())
                .for_each(|model_name| {
                    material_map_by_model
                        .entry(model_name)
                        .or_default()
                        .push(material.clone());
                });
        }

        // shadowing without mutability
        let material_map_by_model = material_map_by_model;

        let model_files: Vec<_> = std::fs::read_dir(Paths::MODELS)
            .context("モデルディレクトリの読み込みに失敗")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .collect();

        println!("  ✓ {} 個のモデルファイルを検出", model_files.len());

        let mut models = Vec::<ModelInfo>::new();

        for entry in model_files {
            let model_path = entry.path();
            let model_name = model_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            if model_name.is_empty() {
                eprintln!("  ✗ 無効なモデルファイル名: {:?}", model_path);
                continue;
            }

            let texture_path = Paths::texture_path(&model_name);
            if !texture_path.exists() {
                eprintln!("  ✗ テクスチャファイルが存在しません: {:?}", texture_path);
                continue;
            }

            let added_date =
                find_git_added_data(&model_path).context("Gitメタデータの取得に失敗")?;
            let animation_metadata = find_animation_metadata(&model_name)
                .context("アニメーションメタデータの取得に失敗")?;

            let model_info = ModelInfo {
                name: model_name.clone(),
                materials: material_map_by_model
                    .get(&model_name)
                    .cloned()
                    .unwrap_or_default(),
                texture_path: texture_path.to_string_lossy().to_string(),
                added_date,
                animation: animation_metadata,
            };

            models.push(model_info);
        }

        let models_data = ModelsData {
            count: models.len(),
            models,
        };

        write_json(&self.output, &models_data).with_context(|| {
            format!(
                "モデルメタデータの書き込みに失敗: {}",
                self.output.display()
            )
        })?;

        Ok(())
    }
}

/// Get git metadata for a file
fn find_git_added_data(file_path: &Path) -> anyhow::Result<String> {
    // Try to get git log for the file
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

        if !log.is_empty() {
            let log = log.lines().next().unwrap_or(log);
            let parts: Vec<&str> = log.split('|').collect();
            if !parts.is_empty() {
                return Ok(parts[0].to_string());
            }
        }
    }

    // Fallback to file modification time
    if let Ok(metadata) = std::fs::metadata(file_path)
        && let Ok(modified) = metadata.modified()
    {
        let datetime: chrono::DateTime<chrono::Utc> = modified.into();
        return Ok(datetime.to_rfc3339());
    }

    Ok(chrono::Utc::now().to_rfc3339())
}

fn find_animation_metadata(model_name: &str) -> anyhow::Result<Option<AnimationMetadata>> {
    let mcmeta_path = Paths::animation_path(model_name);

    if !mcmeta_path.exists() {
        return Ok(None);
    }

    let info = read_json::<AnimationInfo>(&mcmeta_path).with_context(|| {
        format!(
            "アニメーションメタデータの読み込みに失敗: {}",
            mcmeta_path.display()
        )
    })?;

    // Get frame count from image dimensions
    let image_path = Paths::texture_path(model_name);
    let frame_count = if let Ok(img) = image::open(&image_path) {
        let (width, height) = img.dimensions();
        height / width
    } else {
        anyhow::bail!(
            "アニメーション画像の読み込みに失敗: {}",
            image_path.display()
        );
    };

    Ok(Some(AnimationMetadata {
        frame_count,
        frametime: info.animation.frametime,
    }))
}
