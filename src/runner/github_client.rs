use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Serialize;

/// GitHub API client for Actions
pub struct GitHubClient {
    client: Client,
    token: String,
    owner: String,
    repo: String,
}

#[derive(Debug, Serialize)]
struct CommentRequest {
    body: String,
}

#[derive(Debug, Serialize)]
struct ReactionRequest {
    content: String,
}

#[derive(Debug, Serialize)]
struct IssueStateRequest {
    state: String,
}

impl GitHubClient {
    /// Create new GitHub client from environment
    pub fn from_env() -> Result<Self> {
        let token =
            std::env::var("GITHUB_TOKEN").context("GITHUB_TOKEN environment variable not set")?;
        let owner =
            std::env::var("REPO_OWNER").context("REPO_OWNER environment variable not set")?;
        let repo = std::env::var("REPO_NAME").context("REPO_NAME environment variable not set")?;

        let client = Client::builder()
            .user_agent("OfroPack-GitHub-Actions")
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            token,
            owner,
            repo,
        })
    }

    /// Create a comment on an issue
    pub fn comment_issue(&self, issue_number: u64, body: &str) -> Result<()> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            self.owner, self.repo, issue_number
        );

        let request = CommentRequest {
            body: body.to_string(),
        };

        self.post_request(&url, &request)
            .context("Failed to post comment")?;

        println!("✓ Comment posted to issue #{}", issue_number);
        Ok(())
    }

    /// Add a reaction to an issue
    pub fn react_issue(&self, issue_number: u64, reaction: &str) -> Result<()> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/reactions",
            self.owner, self.repo, issue_number
        );

        let request = ReactionRequest {
            content: reaction.to_string(),
        };

        self.post_request(&url, &request)
            .context("Failed to add reaction")?;

        println!("✓ Reaction '{}' added to issue #{}", reaction, issue_number);
        Ok(())
    }

    /// Close an issue
    pub fn close_issue(&self, issue_number: u64) -> Result<()> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            self.owner, self.repo, issue_number
        );

        let request = IssueStateRequest {
            state: "closed".to_string(),
        };

        self.patch_request(&url, &request)
            .context("Failed to close issue")?;

        println!("✓ Issue #{} closed", issue_number);
        Ok(())
    }

    /// Make POST request to GitHub API
    fn post_request<T: Serialize>(&self, url: &str, body: &T) -> Result<()> {
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(body)
            .send()
            .context("Failed to send POST request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("API request failed ({}): {}", status, body);
        }

        Ok(())
    }

    /// Make PATCH request to GitHub API
    fn patch_request<T: Serialize>(&self, url: &str, body: &T) -> Result<()> {
        let response = self
            .client
            .patch(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(body)
            .send()
            .context("Failed to send PATCH request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("API request failed ({}): {}", status, body);
        }

        Ok(())
    }
}
