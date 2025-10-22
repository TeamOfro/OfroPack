use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

use crate::{
    paths::Paths,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_path: Option<String>,
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
        let material_map_by_model = Self::build_material_map()?;
        let model_files = Self::collect_model_files()?;

        println!("  ✓ {} 個のモデルファイルを検出", model_files.len());

        let models: Vec<ModelInfo> = model_files
            .into_iter()
            .filter_map(|entry| Self::process_model_file(entry, &material_map_by_model).ok())
            .collect();

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

impl Models {
    fn extract_file_stem(path: &Path) -> Option<String> {
        path.file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
    }

    fn build_material_map() -> anyhow::Result<HashMap<String, Vec<String>>> {
        let mut material_map_by_model: HashMap<String, Vec<String>> = HashMap::new();

        for entry in std::fs::read_dir(Paths::ITEMS)
            .context("アイテムディレクトリの読み込みに失敗")?
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
        {
            let material_path = entry.path();
            let Some(material) = Self::extract_file_stem(&material_path) else {
                eprintln!("  ✗ 無効なアイテムファイル名: {:?}", material_path);
                continue;
            };

            let item_resource = read_json::<ItemResource>(&material_path).with_context(|| {
                format!(
                    "アイテムリソースの読み込みに失敗: {}",
                    material_path.display()
                )
            })?;

            for case in &item_resource.model.cases {
                material_map_by_model
                    .entry(case.when.clone())
                    .or_default()
                    .push(material.clone());
            }
        }

        Ok(material_map_by_model)
    }

    fn collect_model_files() -> anyhow::Result<Vec<std::fs::DirEntry>> {
        std::fs::read_dir(Paths::MODELS)
            .context("モデルディレクトリの読み込みに失敗")?
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    fn process_model_file(
        entry: std::fs::DirEntry,
        material_map_by_model: &HashMap<String, Vec<String>>,
    ) -> anyhow::Result<ModelInfo> {
        let model_path = entry.path();
        let Some(model_name) = Self::extract_file_stem(&model_path) else {
            eprintln!("  ✗ 無効なモデルファイル名: {:?}", model_path);
            anyhow::bail!("Invalid model file name");
        };

        let texture_path = Paths::texture_path(&model_name);
        let texture_path_dir = Paths::texture_path_dir(&model_name);
        let texture_path = if texture_path_dir.is_dir() {
            None
        } else if !texture_path.exists() {
            eprintln!("  ✗ テクスチャファイルが存在しません: {:?}", texture_path);
            anyhow::bail!("Texture file not found");
        } else {
            Some(texture_path)
        };

        let added_date = find_git_added_data(&model_path).context("Gitメタデータの取得に失敗")?;
        let animation_metadata =
            find_animation_metadata(&model_name).context("アニメーションメタデータの取得に失敗")?;

        Ok(ModelInfo {
            name: model_name.clone(),
            materials: material_map_by_model
                .get(&model_name)
                .cloned()
                .unwrap_or_default(),
            texture_path: texture_path.map(|v| v.to_string_lossy().to_string()),
            added_date,
            animation: animation_metadata,
        })
    }
}

/// Helper trait for method chaining
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

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
