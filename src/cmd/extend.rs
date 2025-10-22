use anyhow::Context;

use crate::{
    paths::Paths,
    schema::items::{ItemCase, ItemResource},
    utils::json::{read_json, write_json},
    validation::should_snake_case,
};

/// 🔧 既存モデルにマテリアルを追加
///
/// 既に存在するカスタムモデルデータに、新しいマテリアルを適用します。
/// モデルファイルは既に存在している必要があります。
#[derive(clap::Parser, Debug)]
#[command(
    about = "既存モデルにマテリアルを追加",
    long_about = "既に作成済みのカスタムモデルデータに、新しいマテリアルを適用します。\n\n\
                  モデルファイル（JSON）は既に存在している必要があります。\n\
                  テクスチャファイルの存在は不要です（3Dモデルの場合など）。"
)]
pub struct Extend {
    /// カンマ区切りのマテリアルリスト
    ///
    /// 例: diamond_axe,iron_sword,golden_pickaxe
    #[arg(
        short,
        long,
        value_delimiter = ',',
        required = true,
        value_name = "MATERIALS",
        help = "追加するマテリアル（カンマ区切り）"
    )]
    pub materials: Vec<String>,

    /// カスタムモデルデータ名
    ///
    /// 既に存在するカスタムモデルデータの名前を指定します。
    #[arg(
        short,
        long,
        required = true,
        value_name = "NAME",
        help = "既存のカスタムモデルデータ名"
    )]
    pub custom_model_data: String,
}

impl super::Run for Extend {
    fn run(&self) -> anyhow::Result<()> {
        println!("\n🔧 マテリアル拡張を開始します...\n");

        if self.materials.is_empty() {
            anyhow::bail!("少なくとも1つのmaterialを指定してください。");
        }
        should_snake_case(&self.custom_model_data)?;
        self.materials
            .iter()
            .try_for_each(|material| should_snake_case(material))?;

        // マテリアルの存在検証（items_textures.json に基づく）
        crate::utils::add::validate_materials(&self.materials)?;

        // モデルは存在する必要がある（テクスチャの存在は不要）
        let model_path = Paths::model_path(&self.custom_model_data);
        if !model_path.exists() {
            anyhow::bail!(
                "❌ モデルファイルが存在しません: {}",
                model_path.to_string_lossy()
            );
        }

        println!("📋 対象モデル: {}", self.custom_model_data);
        println!("📦 追加マテリアル: {}\n", self.materials.join(", "));

        for material in &self.materials {
            println!("  ➜ マテリアル '{}' に適用中...", material);

            extend_material(&self.custom_model_data, material).with_context(|| {
                format!(
                    "マテリアル '{}' への custom_model_data '{}' の追加に失敗しました",
                    material, self.custom_model_data
                )
            })?;
        }

        let added_materials = self.materials.join(", ");
        println!(
            "\n✅ マテリアル [{}] に custom_model_data '{}' を追加しました\n",
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
        let mapping = crate::utils::materials::MaterialMapping::load()?;
        let fallback = mapping.resolve_fallback_model_path(material)?;
        ItemResource::new_with_fallback(&fallback)
    };

    if item_resource
        .model
        .cases
        .iter()
        .any(|case| case.when == custom_model_data)
    {
        println!(
            "  ⚠️  custom_model_data '{}' はすでにマテリアル '{}' に適用されています（スキップ）",
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

    println!("  ✓ 追加完了");

    Ok(())
}
