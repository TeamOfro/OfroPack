use anyhow::Result;

use crate::cmd::{Add, Run};
use crate::processor::Processor;

impl Run for Add {
    fn run(&self) -> Result<()> {
        // Validate that we're in a resource pack directory
        if !std::path::Path::new("assets").exists() {
            anyhow::bail!(
                "assetsディレクトリが存在しません。\n\
                Minecraftリソースパックのルートディレクトリで実行してください。"
            );
        }

        // Validation
        if self.materials.is_empty() {
            anyhow::bail!(
                "少なくとも1つのマテリアルを指定してください\n\
                例: -m diamond_axe,iron_sword"
            );
        }

        if !self.path_to_image.exists() {
            anyhow::bail!(
                "画像ファイルが見つかりません: {}",
                self.path_to_image.display()
            );
        }

        // Get custom model data name
        let custom_model_data = match &self.custom_model_data {
            Some(name) => name.to_lowercase(),
            None => self
                .path_to_image
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("画像ファイル名を取得できません"))?
                .to_string(),
        };

        // Validate custom model data name
        if !custom_model_data
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            anyhow::bail!("カスタムモデルデータ名は小文字英数字とアンダースコアのみ使用できます");
        }

        // Normalize materials
        let normalized_materials: Vec<String> =
            self.materials.iter().map(|m| m.to_lowercase()).collect();

        // Process
        let processor = Processor::new(custom_model_data);
        processor.add_with_texture(&normalized_materials, &self.path_to_image)?;

        Ok(())
    }
}
