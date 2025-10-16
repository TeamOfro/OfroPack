use anyhow::{Context, Result};
use std::path::PathBuf;

use super::{GitHubClient, ImageDownloader, IssueParser, PreviewGenerator};
use crate::processor::Processor;

/// Orchestrates the entire issue processing workflow
pub struct IssueProcessor {
    github_client: GitHubClient,
    image_downloader: ImageDownloader,
}

pub struct ProcessResult {
    pub preview_url: String,
    pub custom_model_data: String,
}

impl IssueProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            github_client: GitHubClient::from_env()?,
            image_downloader: ImageDownloader::new()?,
        })
    }

    /// Process issue: parse, download, validate, add model, generate preview
    pub fn process(&self, issue_number: u64, issue_body: &str) -> Result<ProcessResult> {
        println!("\n=== Issue #{}の処理を開始 ===\n", issue_number);

        // Step 1: Add rocket reaction
        println!("🚀 処理を開始します...");
        self.github_client
            .react_issue(issue_number, "rocket")
            .context("Reactionの追加に失敗しました")?;

        // Step 2: Parse issue body
        println!("\n📝 Issueを解析中...");
        let parsed = IssueParser::parse(issue_body).context("Issueの解析に失敗しました")?;
        println!("  マテリアル: {}", parsed.materials.join(", "));
        println!("  カスタムモデルデータ: {}", parsed.custom_model_data);
        println!("  画像URL: {}", parsed.image_url);

        // Step 3: Download and validate image
        println!("\n⬇️  画像をダウンロード中...");
        let image_file = PathBuf::from(format!("{}.png", parsed.custom_model_data));
        self.image_downloader
            .download(&parsed.image_url, &image_file)
            .context("画像のダウンロードに失敗しました")?;

        // Step 4: Process custom model data
        println!("\n⚙️  カスタムモデルを処理中...");
        let processor = Processor::new(parsed.custom_model_data.clone());
        processor
            .add_with_texture(&parsed.materials, &image_file)
            .context("カスタムモデルの追加に失敗しました")?;
        println!("✓ カスタムモデルの追加が完了しました");

        // Step 5: Generate preview
        println!("\n🖼️  プレビュー画像を生成中...");
        let preview_generator = PreviewGenerator::new(PathBuf::from("preview"));
        let texture_path = PathBuf::from(format!(
            "assets/minecraft/textures/item/{}.png",
            parsed.custom_model_data
        ));

        if !texture_path.exists() {
            anyhow::bail!(
                "テクスチャファイルが見つかりません: {}",
                texture_path.display()
            );
        }

        let preview_path = preview_generator
            .generate(&texture_path, &parsed.custom_model_data)
            .context("プレビュー画像の生成に失敗しました")?;

        // Get environment variables for URL generation
        let repo_owner = std::env::var("REPO_OWNER").unwrap_or_else(|_| "unknown".to_string());
        let repo_name = std::env::var("REPO_NAME").unwrap_or_else(|_| "unknown".to_string());
        let pr_branch = std::env::var("PR_BRANCH")
            .unwrap_or_else(|_| format!("custom-model/issue-{}", issue_number));

        let preview_url =
            PreviewGenerator::generate_url(&repo_owner, &repo_name, &pr_branch, &preview_path);

        println!("✓ プレビュー画像の生成が完了しました");

        println!("\n=== 処理が正常に完了しました ===\n");

        Ok(ProcessResult {
            preview_url,
            custom_model_data: parsed.custom_model_data,
        })
    }

    /// Post success comment with preview
    pub fn post_success(&self, issue_number: u64, pr_number: u64, preview_url: &str) -> Result<()> {
        let comment = format!(
            r#"## ✅ カスタムモデルの処理が完了しました！

**Pull Request:** #{}

### プレビュー（256×256、ピクセルパーフェクト）

![Custom Model Preview]({})

このカスタムモデルをリソースパックに追加するため、PRをレビューしてマージしてください。"#,
            pr_number, preview_url
        );

        self.github_client
            .comment_issue(issue_number, &comment)
            .context("成功コメントの投稿に失敗しました")?;

        self.github_client
            .react_issue(issue_number, "+1")
            .context("Reactionの追加に失敗しました")?;

        Ok(())
    }

    /// Post failure comment and close issue
    pub fn post_failure(
        &self,
        issue_number: u64,
        error_message: &str,
        workflow_url: &str,
    ) -> Result<()> {
        let comment = format!(
            r#"## ❌ カスタムモデルの処理に失敗しました

ワークフローでエラーが発生しました。詳細は[ワークフローログ]({})を確認してください。

### エラー内容

```
{}
```

### 次のステップ

1. ログを確認してエラーの原因を特定する
2. 入力内容を修正する
3. **新しいIssueを作成する**（正しい情報を入力）

⚠️ **注意:** このIssueを編集しても再実行されません。新しいIssueを作成してください。"#,
            workflow_url, error_message
        );

        self.github_client
            .comment_issue(issue_number, &comment)
            .context("エラーコメントの投稿に失敗しました")?;

        self.github_client
            .react_issue(issue_number, "-1")
            .context("Reactionの追加に失敗しました")?;

        self.github_client
            .close_issue(issue_number)
            .context("Issueのクローズに失敗しました")?;

        Ok(())
    }
}
