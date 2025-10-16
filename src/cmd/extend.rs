use anyhow::Result;

use crate::cmd::{Extend, Run};
use crate::processor::Processor;

impl Run for Extend {
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
            anyhow::bail!("少なくとも1つのマテリアルを指定してください");
        }

        if !self
            .custom_model_data
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            anyhow::bail!("カスタムモデルデータ名は小文字英数字とアンダースコアのみ使用できます");
        }

        // Normalize
        let custom_model_data = self.custom_model_data.to_lowercase();
        let normalized_materials: Vec<String> =
            self.materials.iter().map(|m| m.to_lowercase()).collect();

        // Process
        let processor = Processor::new(custom_model_data);
        processor.extend_materials(&normalized_materials)?;

        Ok(())
    }
}
