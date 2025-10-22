use anyhow::Result;

use crate::{cmd::Run, pipeline::runner::process_issue::IssueProcessor, types::IssueType};

#[derive(clap::Parser, Debug)]
pub struct ProcessIssue {
    #[arg(long)]
    issue_number: u64,

    #[arg(long)]
    issue_type: IssueType,

    #[arg(long)]
    body: String,

    #[arg(long)]
    actor: String,

    #[arg(long)]
    actor_email: String,
}

impl Run for ProcessIssue {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        processor.process(
            self.issue_number,
            self.issue_type,
            &self.body,
            &self.actor,
            &self.actor_email,
        )?;
        Ok(())
    }
}
