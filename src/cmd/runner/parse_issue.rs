use anyhow::Result;

use crate::{
    cmd::Run,
    constants::IssueType,
    pipeline::runner::issue_parser::{IssueParser, ParsedIssue},
};

#[derive(clap::Parser, Debug)]
pub struct ParseIssue {
    #[arg(long)]
    body: String,

    #[arg(long)]
    issue_type: IssueType,
}

impl Run for ParseIssue {
    fn run(&self) -> Result<()> {
        let parsed = IssueParser::parse(&self.body, self.issue_type)?;
        match parsed {
            ParsedIssue::AddCustomModel {
                materials,
                custom_model_data,
                image_url,
                animation,
            } => {
                println!("issue_type=add");
                println!("materials={}", materials.join(","));
                println!("custom_model_data={}", custom_model_data);
                println!("image_url={}", image_url);
                if let Some(anim) = animation {
                    println!("frametime={}", anim.animation.frametime);
                }
            }
            ParsedIssue::Extend {
                materials,
                custom_model_data,
            } => {
                println!("issue_type=extend");
                println!("materials={}", materials.join(","));
                println!("custom_model_data={}", custom_model_data);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
}
