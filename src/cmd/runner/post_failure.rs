use anyhow::Result;

use crate::{cmd::Run, pipeline::runner::process_issue::IssueProcessor};

#[derive(clap::Parser, Debug)]
pub struct PostFailure {
    #[arg(long)]
    issue_number: u64,

    #[arg(long)]
    error_message: String,

    #[arg(long)]
    workflow_url: String,
}

impl Run for PostFailure {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        processor.post_failure(self.issue_number, &self.error_message, &self.workflow_url)?;
        Ok(())
    }
}
