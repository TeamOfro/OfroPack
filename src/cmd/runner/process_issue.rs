use anyhow::Result;

use crate::{
    cmd::Run,
    constants::IssueType,
    pipeline::runner::process_issue::{IssueProcessor, ProcessResult},
};

#[derive(clap::Parser, Debug)]
pub struct ProcessIssue {
    #[arg(long)]
    issue_number: u64,

    #[arg(long)]
    issue_type: IssueType,

    #[arg(long)]
    body: String,
}

impl Run for ProcessIssue {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        let result: ProcessResult = processor.process(self.issue_number, self.issue_type, &self.body)?;

        if let Some(preview_url) = result.preview_url {
            println!("PREVIEW_URL={}", preview_url);
        }
        if let Some(materials) = result.added_materials {
            println!("ADDED_MATERIALS={}", materials.join(","));
        }
        println!("CUSTOM_MODEL_DATA={}", result.custom_model_data);

        Ok(())
    }
}
