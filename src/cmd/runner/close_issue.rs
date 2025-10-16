use anyhow::Result;

use crate::runner::GitHubClient;

pub fn run(issue_number: u64) -> Result<()> {
    let client = GitHubClient::from_env()?;
    client.close_issue(issue_number)?;
    Ok(())
}
