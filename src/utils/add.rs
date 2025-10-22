use std::path::Path;

use anyhow::Context;

use crate::{
    paths::Paths,
    schema::{
        animation::AnimationInfo,
        items::{ItemCase, ItemResource},
        models::ItemModel,
    },
    types::ItemModelParent,
    utils::{
        json::{read_json, write_json},
        materials::MaterialMapping,
    },
    validation::should_snake_case,
};

pub fn validate_materials(materials: &[String]) -> anyhow::Result<()> {
    if materials.is_empty() {
        anyhow::bail!("少なくとも1つのmaterialを指定してください。");
    }
    materials.iter().try_for_each(|m| should_snake_case(m))?;

    let mapping = MaterialMapping::load()?;
    let unknown: Vec<_> = materials
        .iter()
        .filter(|m| !mapping.contains(m))
        .cloned()
        .collect();
    if !unknown.is_empty() {
        anyhow::bail!("未対応のmaterialが指定されました: {}", unknown.join(", "));
    }
    Ok(())
}

pub fn infer_or_validate_name(
    name: &Option<String>,
    path_to_image: &Path,
) -> anyhow::Result<String> {
    let name = match name {
        Some(n) => n.to_lowercase(),
        None => path_to_image
            .file_stem()
            .and_then(|s| s.to_str())
            .context("画像ファイル名の取得に失敗")?
            .to_string(),
    };
    should_snake_case(&name)?;
    Ok(name)
}

pub fn ensure_not_exists_2d(custom_model_data: &str) -> anyhow::Result<()> {
    let model_path = Paths::model_path(custom_model_data);
    let texture_path = Paths::texture_path(custom_model_data);

    if model_path.exists() {
        anyhow::bail!(
            "モデルファイルが既に存在します: {}",
            model_path.to_string_lossy()
        );
    }
    if texture_path.exists() {
        anyhow::bail!(
            "テクスチャファイルが既に存在します: {}",
            texture_path.to_string_lossy()
        );
    }
    Ok(())
}

pub fn write_new_item_model(
    parent: ItemModelParent,
    custom_model_data: &str,
) -> anyhow::Result<()> {
    let model_path = Paths::model_path(custom_model_data);
    let item_model = ItemModel::new(parent, custom_model_data);
    write_json(&model_path, &item_model)
        .with_context(|| format!("モデルファイルの書き込みに失敗: {}", model_path.display()))
}

pub fn update_materials(materials: &[String], custom_model_data: &str) -> anyhow::Result<()> {
    let case = ItemCase::new(custom_model_data);
    for material in materials {
        let material_path = Paths::item_path(material);
        let mut resource = if material_path.exists() {
            read_json::<ItemResource>(&material_path).with_context(|| {
                format!(
                    "マテリアルファイルの読み込みに失敗: {}",
                    material_path.display()
                )
            })?
        } else {
            let mapping = MaterialMapping::load()?;
            let fallback = mapping.resolve_fallback_model_path(material)?;
            ItemResource::new_with_fallback(&fallback)
        };
        resource.add_case(case.clone());
        write_json(&material_path, &resource).with_context(|| {
            format!(
                "マテリアルファイルの書き込みに失敗: {}",
                material_path.display()
            )
        })?;
        println!("  ✓ マテリアル '{}' に追加完了", material);
    }
    Ok(())
}

pub fn write_new_animation(
    custom_model_data: &str,
    animation_info: &AnimationInfo,
) -> anyhow::Result<()> {
    let animation_path = Paths::animation_path(custom_model_data);
    write_json(&animation_path, &animation_info).with_context(|| {
        format!(
            "アニメーションファイルの書き込みに失敗: {}",
            animation_path.display()
        )
    })
}
