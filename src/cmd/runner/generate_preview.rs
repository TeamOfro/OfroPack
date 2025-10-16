use anyhow::Result;
use std::path::Path;

use crate::runner::PreviewGenerator;

pub fn run(
    source: &Path,
    model_name: &str,
    preview_dir: &Path,
    repo_owner: Option<&str>,
    repo_name: Option<&str>,
    branch: Option<&str>,
) -> Result<()> {
    let generator = PreviewGenerator::new(preview_dir);
    let preview_path = generator.generate(source, model_name)?;

    // Generate URL if repo info provided
    if let (Some(owner), Some(name), Some(br)) = (repo_owner, repo_name, branch) {
        let url = PreviewGenerator::generate_url(owner, name, br, &preview_path);
        println!("preview_url={}", url);
    }

    Ok(())
}
