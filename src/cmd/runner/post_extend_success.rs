use anyhow::Result;

use crate::runner::IssueProcessor;

pub fn run(issue_number: u64, pr_number: u64) -> Result<()> {
    let processor = IssueProcessor::new()?;
    processor.post_extend_success(issue_number, pr_number)?;
    Ok(())
}
