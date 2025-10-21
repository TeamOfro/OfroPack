use anyhow::Context;

use crate::{
    constants::{Paths, should_snake_case},
    schema::items::{ItemCase, ItemResource},
    utils::json::{read_json, write_json},
};

/// 既存のカスタムモデルデータをまだ適用していないmaterialに適用する
#[derive(clap::Parser, Debug)]
pub struct Extend {
    /// カンマ区切りの material list
    #[arg(short, long, value_delimiter = ',', required = true)]
    pub materials: Vec<String>,

    /// カスタムモデルデータ名
    #[arg(short, long, required = true)]
    pub custom_model_data: String,
}

impl super::Run for Extend {
    fn run(&self) -> anyhow::Result<()> {
        if self.materials.is_empty() {
            anyhow::bail!("少なくとも1つのmaterialを指定してください。");
        }
        should_snake_case(&self.custom_model_data)?;
        self.materials
            .iter()
            .try_for_each(|material| should_snake_case(material))?;

        // モデルは存在する必要がある（テクスチャの存在は不要）
        let model_path = Paths::model_path(&self.custom_model_data);
        if !model_path.exists() {
            anyhow::bail!(
                "モデルファイルが存在しません: {}",
                model_path.to_string_lossy()
            );
        }

        for material in &self.materials {
            println!(
                "Adding custom_model_data '{}' to material '{}'",
                self.custom_model_data, material
            );

            extend_material(&self.custom_model_data, material).with_context(|| {
                format!(
                    "Failed to extend material '{}' with custom_model_data '{}'",
                    material, self.custom_model_data
                )
            })?;
        }

        let added_materials = self.materials.join(", ");
        println!(
            "Successfully extended materials [{}] with custom_model_data '{}'",
            added_materials, self.custom_model_data
        );

        Ok(())
    }
}

fn extend_material(custom_model_data: &str, material: &str) -> anyhow::Result<()> {
    let material_path = Paths::item_path(material);

    let mut item_resource = if material_path.exists() {
        read_json::<ItemResource>(&material_path).with_context(|| {
            format!(
                "マテリアルファイルの読み込みに失敗: {}",
                material_path.display()
            )
        })?
    } else {
        ItemResource::new(material)
    };

    if item_resource
        .model
        .cases
        .iter()
        .any(|case| case.when == custom_model_data)
    {
        println!(
            "  WARN: custom_model_data '{}' はすでに material '{}' に適用されています。",
            custom_model_data, material
        );
        return Ok(());
    }

    item_resource
        .model
        .cases
        .push(ItemCase::new(custom_model_data));

    write_json(&material_path, &item_resource).with_context(|| {
        format!(
            "マテリアルファイルの書き込みに失敗: {}",
            material_path.display()
        )
    })?;

    Ok(())
}
