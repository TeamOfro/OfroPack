use std::path::PathBuf;

use anyhow::Result;

use crate::{
    cmd::Run,
    config::{REPO_NAME, REPO_OWNER},
    pipeline::preview_generator::PreviewGenerator,
};

#[derive(clap::Parser, Debug)]
pub struct GeneratePreview {
    #[arg(long)]
    source: PathBuf,

    #[arg(long)]
    model_name: String,

    #[arg(long, default_value = REPO_OWNER)]
    repo_owner: String,

    #[arg(long, default_value = REPO_NAME)]
    repo_name: String,

    #[arg(long, default_value = "main")]
    branch: String,
}

impl Run for GeneratePreview {
    fn run(&self) -> Result<()> {
        let preview_path = PreviewGenerator::generate(&self.source, &self.model_name)?;

        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            self.repo_owner,
            self.repo_name,
            self.branch,
            preview_path.to_string_lossy()
        );
        println!("preview_url={}", url);

        Ok(())
    }
}
