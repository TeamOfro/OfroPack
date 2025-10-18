use std::path::PathBuf;

use anyhow::Result;

use crate::cmd::Run;

#[derive(clap::Parser, Debug)]
pub struct Generates {
    /// models.json の出力パス
    #[arg(long, default_value = "models.json")]
    pub gallery_output: PathBuf,

    /// metadata.json の出力パス
    #[arg(long, default_value = "metadata.json")]
    pub metadata_output: PathBuf,

    /// Zipファイルの出力パス
    #[arg(long, default_value = "OfroPack.zip")]
    pub zip_output: PathBuf,

    /// GitHub Repository owner (for metadata)
    #[arg(long)]
    pub repo_owner: Option<String>,

    /// GitHub Repository name (for metadata)
    #[arg(long)]
    pub repo_name: Option<String>,
}

impl Run for Generates {
    fn run(&self) -> Result<()> {
        println!("🚀 リソースパック生成を開始します\n");

        // 1. Generate gallery (models.json)
        println!("【1/3】ギャラリーデータを生成");
        let gallery = super::cmd::GenerateGallery {
            output: self.gallery_output.clone(),
        };
        gallery.run()?;

        println!();

        // 2. Generate zip
        println!("【2/3】リソースパックを圧縮");
        let zip = super::zip::GenerateZip {
            output: self.zip_output.clone(),
            files: vec![
                "assets/".to_string(),
                "pack.mcmeta".to_string(),
                "pack.png".to_string(),
            ],
        };
        zip.run()?;

        println!();

        // 3. Generate metadata
        println!("【3/3】メタデータを生成");
        let metadata = super::metadata::GenerateMetadata {
            output: self.metadata_output.clone(),
            zip: self.zip_output.clone(),
            repo_owner: self.repo_owner.clone(),
            repo_name: self.repo_name.clone(),
        };
        metadata.run()?;

        println!("\n✨ すべての生成が完了しました！");

        Ok(())
    }
}
