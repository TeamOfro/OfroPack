use anyhow::Result;
use std::path::Path;

use crate::runner::PreviewGenerator;

pub fn run(
    source: &Path,
    model_name: &str,
    preview_dir: &Path,
    repo_owner: &str,
    repo_name: &str,
    branch: &str,
) -> Result<()> {
    let generator = PreviewGenerator::new(preview_dir);
    let preview_path = generator.generate(source, model_name)?;

    let url = PreviewGenerator::generate_url(repo_owner, repo_name, branch, &preview_path);
    println!("preview_url={}", url);

    Ok(())
}
