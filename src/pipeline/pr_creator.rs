use anyhow::{Context, Result};
use std::process::Command;

use crate::pipeline::github_client::GitHubClient;

pub struct PrCreator {
    github_client: GitHubClient,
}

impl PrCreator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            github_client: GitHubClient::from_env()?,
        })
    }

    fn run_git_command(&self, args: &[&str], error_context: &str) -> Result<()> {
        let output = Command::new("git")
            .args(args)
            .output()
            .with_context(|| format!("{}の実行に失敗しました", error_context))?;

        if !output.status.success() {
            anyhow::bail!(
                "{}に失敗しました: {}",
                error_context,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    pub fn create_pr(
        &self,
        branch_name: &str,
        title: &str,
        body: &str,
        author_name: &str,
        author_email: &str,
    ) -> Result<u64> {
        println!("\n📝 プルリクエストを作成中...");

        // 1. Git設定
        self.configure_git(author_name, author_email)?;

        // 2. Git add
        self.git_add()?;

        // 3. Git commit
        self.git_commit(title)?;

        // 4. Git checkout -b & push
        self.git_push(branch_name)?;

        // 5. GitHub APIでPR作成
        let pr_number = self
            .github_client
            .create_pull_request(branch_name, "main", title, body)?;

        Ok(pr_number)
    }

    fn configure_git(&self, author_name: &str, author_email: &str) -> Result<()> {
        self.run_git_command(&["config", "user.name", author_name], "Git user.nameの設定")?;
        self.run_git_command(
            &["config", "user.email", author_email],
            "Git user.emailの設定",
        )?;
        Ok(())
    }

    fn git_add(&self) -> Result<()> {
        self.run_git_command(&["add", "."], "git add")?;
        println!("  ✓ ファイルをステージングしました");
        Ok(())
    }

    fn git_commit(&self, message: &str) -> Result<()> {
        self.run_git_command(&["commit", "-m", message], "git commit")?;
        println!("  ✓ コミットを作成しました");
        Ok(())
    }

    fn git_push(&self, branch_name: &str) -> Result<()> {
        // Checkout新しいブランチ
        self.run_git_command(&["checkout", "-b", branch_name], "git checkout -b")?;

        // Push
        self.run_git_command(&["push", "-u", "origin", branch_name], "git push")?;

        println!("  ✓ ブランチ '{branch_name}' をプッシュしました");
        Ok(())
    }
}
