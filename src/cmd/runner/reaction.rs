use anyhow::Context;

use crate::{cmd::Run, constants::GithubReaction};

#[derive(clap::Parser, Debug)]
pub struct Reaction {
    /// Issue number
    #[arg(long)]
    issue_number: u64,

    #[arg(long)]
    reaction: GithubReaction,
}

impl Run for Reaction {
    fn run(&self) -> anyhow::Result<()> {
        let github_client = crate::pipeline::github_client::GitHubClient::from_env()?;
        github_client
            .react_issue(self.issue_number, self.reaction)
            .context("Failed to add reaction to GitHub issue")?;
        Ok(())
    }
}
