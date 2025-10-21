use anyhow::Context;

use crate::cmd::Run;

#[derive(clap::Parser, Debug)]
pub struct Comment {
    /// Issue番号
    #[arg(long)]
    issue_number: u64,

    /// コメント本文（Markdown）
    #[arg(long)]
    body: String,
}

impl Run for Comment {
    fn run(&self) -> anyhow::Result<()> {
        let github_client = crate::pipeline::github_client::GitHubClient::from_env()?;
        github_client
            .comment_issue(self.issue_number, &self.body)
            .context("GitHub Issueへのコメント投稿に失敗しました")?;
        Ok(())
    }
}
