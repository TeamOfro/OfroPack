use anyhow::Result;

use crate::{constants::IssueType, runner::IssueProcessor};

pub fn run(issue_number: u64, issue_type: IssueType, body: &str) -> Result<()> {
    let processor = IssueProcessor::new()?;
    let result = processor.process(issue_number, issue_type, body)?;

    // Output for GitHub Actions
    if let Some(preview_url) = result.preview_url {
        println!("PREVIEW_URL={}", preview_url);
    }
    println!("CUSTOM_MODEL_DATA={}", result.custom_model_data);

    Ok(())
}
