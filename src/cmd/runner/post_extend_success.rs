use anyhow::Result;

use crate::{cmd::Run, pipeline::runner::process_issue::IssueProcessor};

#[derive(clap::Parser, Debug)]
pub struct PostExtendSuccess {
    #[arg(long)]
    issue_number: u64,

    #[arg(long)]
    pr_number: u64,

    #[arg(long, value_delimiter = ',')]
    materials: Vec<String>,
}

impl Run for PostExtendSuccess {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        processor.post_extend_success(self.issue_number, self.pr_number, &self.materials)?;
        Ok(())
    }
}
