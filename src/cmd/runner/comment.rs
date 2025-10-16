use anyhow::Result;

use crate::runner::GitHubClient;

pub fn run(issue_number: u64, body: &str) -> Result<()> {
    let client = GitHubClient::from_env()?;
    client.comment_issue(issue_number, body)?;
    Ok(())
}
