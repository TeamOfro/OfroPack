use anyhow::{Context, Result};
use serde::Serialize;
use ureq::{Agent, RequestBuilder, typestate::WithBody};

use crate::{
    config::{REPO_NAME, REPO_OWNER},
    types::GithubReaction,
};

/// GitHub API client for Actions
pub struct GitHubClient {
    client: Agent,
    token: String,
}

#[derive(Debug, Serialize)]
struct CommentRequest {
    body: String,
}

#[derive(Debug, Serialize)]
struct ReactionRequest {
    content: GithubReaction,
}

#[derive(Debug, Serialize)]
struct IssueStateRequest {
    state: String,
}

impl GitHubClient {
    /// Create new GitHub client from environment
    pub fn from_env() -> Result<Self> {
        let token =
            std::env::var("GITHUB_TOKEN").context("環境変数 GITHUB_TOKEN が設定されていません")?;
        let client = Agent::config_builder()
            .user_agent("OfroPack-GitHub-Actions")
            .build()
            .into();

        Ok(Self { client, token })
    }

    fn base_issue_url(issue_number: u64) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            REPO_OWNER, REPO_NAME, issue_number
        )
    }

    fn comments_url(issue_number: u64) -> String {
        format!("{}/comments", Self::base_issue_url(issue_number))
    }

    fn reactions_url(issue_number: u64) -> String {
        format!("{}/reactions", Self::base_issue_url(issue_number))
    }

    /// Create a comment on an issue
    pub fn comment_issue(&self, issue_number: u64, body: &str) -> Result<()> {
        let request = CommentRequest {
            body: body.to_string(),
        };

        self.post_request(&GitHubClient::comments_url(issue_number), &request)
            .context("コメントの投稿に失敗しました")?;

        println!("✓ Issue #{} にコメントを投稿しました", issue_number);
        Ok(())
    }

    /// Add a reaction to an issue
    pub fn react_issue(&self, issue_number: u64, reaction: GithubReaction) -> Result<()> {
        let request = ReactionRequest { content: reaction };

        self.post_request(&GitHubClient::reactions_url(issue_number), &request)
            .context("リアクションの追加に失敗しました")?;

        println!(
            "✓ Issue #{} にリアクション '{}' を追加しました",
            issue_number, reaction
        );
        Ok(())
    }

    /// Close an issue
    pub fn close_issue(&self, issue_number: u64) -> Result<()> {
        let request = IssueStateRequest {
            state: "closed".to_string(),
        };

        self.patch_request(&GitHubClient::base_issue_url(issue_number), &request)
            .context("Issueのクローズに失敗しました")?;

        println!("✓ Issue #{} をクローズしました", issue_number);
        Ok(())
    }

    /// Make POST request to GitHub API
    fn post_request<T: Serialize>(&self, url: &str, body: &T) -> Result<()> {
        let request = self.client.post(url);
        self.inner_request(request, body, "POST")
    }

    /// Make PATCH request to GitHub API
    fn patch_request<T: Serialize>(&self, url: &str, body: &T) -> Result<()> {
        let request = self.client.patch(url);
        self.inner_request(request, body, "PATCH")
    }

    fn inner_request<T: Serialize>(
        &self,
        request: RequestBuilder<WithBody>,
        body: &T,
        method: &str,
    ) -> Result<()> {
        let mut response = request
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send_json(body)
            .with_context(|| format!("{} リクエストの送信に失敗しました", method))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.body_mut().read_to_string().unwrap_or_default();
            anyhow::bail!("APIリクエストに失敗しました ({}): {}", status, body);
        }

        Ok(())
    }
}
