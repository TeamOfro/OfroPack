use anyhow::Context;

use crate::cmd::Run;

#[derive(clap::Parser, Debug)]
pub struct Comment {
    /// Issue number
    #[arg(long)]
    issue_number: u64,

    /// Comment body (Markdown format)
    #[arg(long)]
    body: String,
}

impl Run for Comment {
    fn run(&self) -> anyhow::Result<()> {
        let github_client = crate::pipeline::github_client::GitHubClient::from_env()?;
        github_client
            .comment_issue(self.issue_number, &self.body)
            .context("Failed to post comment to GitHub issue")?;
        Ok(())
    }
}
