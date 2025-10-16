use anyhow::Result;

use crate::runner::IssueProcessor;

pub fn run(issue_number: u64, error_message: &str, workflow_url: &str) -> Result<()> {
    let processor = IssueProcessor::new()?;
    processor.post_failure(issue_number, error_message, workflow_url)?;
    Ok(())
}
