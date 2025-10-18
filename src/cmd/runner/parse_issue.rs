use anyhow::Result;

use crate::{constants::IssueType, runner::IssueParser};

pub fn run(body: &str, issue_type: IssueType) -> Result<()> {
    let parsed = IssueParser::parse(body, issue_type)?;
    println!("{}", IssueParser::output_github_actions(&parsed));
    Ok(())
}
