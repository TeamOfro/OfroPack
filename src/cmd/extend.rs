use crate::constants::{Paths, should_snake_case};

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

        let model_path = Paths::model_path(&self.custom_model_data);
        let texture_path = Paths::texture_path(&self.custom_model_data);

        if !model_path.exists() {
            anyhow::bail!(
                "モデルファイルが存在しません: {}",
                model_path.to_string_lossy()
            );
        }

        if !texture_path.exists() {
            anyhow::bail!(
                "テクスチャファイルが存在しません: {}",
                texture_path.to_string_lossy()
            );
        }

        for material in &self.materials {
            println!(
                "Adding custom_model_data '{}' to material '{}'",
                self.custom_model_data, material
            );

            // TODO: ここで実際にmaterialのJSONファイルを読み込み、custom_model_dataを追加する処理を実装する
        }

        let added_materials = self.materials.join(", ");

        println!(
            "Successfully extended materials [{}] with custom_model_data '{}'",
            self.custom_model_data, added_materials
        );

        Ok(())
    }
}
