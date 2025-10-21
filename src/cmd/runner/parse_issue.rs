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
            ParsedIssue::Model {
                materials,
                custom_model_data,
                image_url,
                animation,
                parent,
            } => {
                println!("issue_type=model");
                println!("materials={}", materials.join(","));
                println!("custom_model_data={}", custom_model_data);
                println!("image_url={}", image_url);
                println!("parent={}", parent.as_str());
                if let Some(anim) = animation {
                    println!("frametime={}", anim.animation.frametime);
                }
            }
            ParsedIssue::Model3d {
                materials,
                custom_model_data,
                model_json_url,
                layer_image_urls,
            } => {
                println!("issue_type=model3d");
                println!("materials={}", materials.join(","));
                println!("custom_model_data={}", custom_model_data);
                println!("model_json_url={}", model_json_url);
                println!("layer_image_urls={}", layer_image_urls.join("\n"));
            }
            ParsedIssue::Extend {
                materials,
                custom_model_data,
            } => {
                println!("issue_type=extend");
                println!("materials={}", materials.join(","));
                println!("custom_model_data={}", custom_model_data);
            }
        }
        Ok(())
    }
}
