use anyhow::Result;

use crate::runner::IssueProcessor;

pub fn run(issue_number: u64, pr_number: u64, preview_url: &str) -> Result<()> {
    let processor = IssueProcessor::new()?;
    processor.post_success(issue_number, pr_number, preview_url)?;
    Ok(())
}
