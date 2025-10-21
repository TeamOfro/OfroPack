use anyhow::Result;

use crate::{cmd::Run, pipeline::runner::process_issue::IssueProcessor};

#[derive(clap::Parser, Debug)]
pub struct PostSuccess {
    #[arg(long)]
    issue_number: u64,

    #[arg(long)]
    pr_number: u64,

    #[arg(long)]
    preview_url: Option<String>,
}

impl Run for PostSuccess {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        processor.post_success(self.issue_number, self.pr_number, self.preview_url.as_deref())?;
        Ok(())
    }
}
