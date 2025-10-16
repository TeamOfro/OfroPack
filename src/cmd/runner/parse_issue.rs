use anyhow::Result;

use crate::runner::IssueParser;

pub fn run(body: &str) -> Result<()> {
    let parsed = IssueParser::parse(body)?;
    println!("{}", IssueParser::output_github_actions(&parsed));
    Ok(())
}
