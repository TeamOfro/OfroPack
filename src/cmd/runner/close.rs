use anyhow::Context;

use crate::cmd::Run;

#[derive(clap::Parser, Debug)]
pub struct Close {
    /// Issue number
    #[arg(long)]
    issue_number: u64,
}

impl Run for Close {
    fn run(&self) -> anyhow::Result<()> {
        let github_client = crate::pipeline::github_client::GitHubClient::from_env()?;
        github_client
            .close_issue(self.issue_number)
            .context("Failed to close GitHub issue")?;
        Ok(())
    }
}
