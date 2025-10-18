use anyhow::{Context, Result};
use std::path::PathBuf;

use super::{GitHubClient, ImageDownloader, IssueParser, PreviewGenerator};
use crate::constants::{IssueType, REPO_NAME, REPO_OWNER};
use crate::runner::issue_parser::ParsedIssueData;
use crate::services::Processor;

/// Orchestrates the entire issue processing workflow
pub struct IssueProcessor {
    github_client: GitHubClient,
    image_downloader: ImageDownloader,
}

pub struct ProcessResult {
    pub preview_url: Option<String>, // None for Extend type
    pub custom_model_data: String,
    pub added_materials: Option<Vec<String>>, // Some for Extend type
}

impl IssueProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            github_client: GitHubClient::from_env()?,
            image_downloader: ImageDownloader::new()?,
        })
    }

    /// Process issue: parse, download (if Add), validate, add/extend model, generate preview (if Add)
    pub fn process(
        &self,
        issue_number: u64,
        issue_type: IssueType,
        issue_body: &str,
    ) -> Result<ProcessResult> {
        println!("\n=== Issue #{}の処理を開始 ===\n", issue_number);

        // Step 1: Add rocket reaction
        println!("🚀 処理を開始します...");
        self.github_client
            .react_issue(issue_number, "rocket")
            .context("Reactionの追加に失敗しました")?;

        // Step 2: Parse issue body
        println!("\n📝 Issueを解析中...");
        let parsed =
            IssueParser::parse(issue_body, issue_type).context("Issueの解析に失敗しました")?;

        match parsed {
            ParsedIssueData::Add {
                materials,
                custom_model_data,
                image_url,
                animation,
            } => {
                println!("  タイプ: Add");
                println!("  マテリアル: {}", materials.join(", "));
                println!("  カスタムモデルデータ: {}", custom_model_data);
                println!("  画像URL: {}", image_url);
                if let Some(anim) = &animation {
                    println!("  Frametime: {}", anim.frametime.get());
                }

                // Step 3: Download and validate image
                println!("\n⬇️  画像をダウンロード中...");
                let image_file = PathBuf::from(format!("{}.png", custom_model_data));
                self.image_downloader
                    .download(&image_url, &image_file)
                    .context("画像のダウンロードに失敗しました")?;

                // Step 4: Process custom model data
                println!("\n⚙️  カスタムモデルを処理中...");
                let processor = Processor::new(custom_model_data.clone());
                processor
                    .add_with_texture(&materials, &image_file, animation)
                    .context("カスタムモデルの追加に失敗しました")?;
                println!("✓ カスタムモデルの追加が完了しました");

                // Step 5: Generate preview
                println!("\n🖼️  プレビュー画像を生成中...");
                let preview_generator = PreviewGenerator::new(PathBuf::from("preview"));
                let texture_path = PathBuf::from(format!(
                    "assets/minecraft/textures/item/{}.png",
                    custom_model_data
                ));

                if !texture_path.exists() {
                    anyhow::bail!(
                        "テクスチャファイルが見つかりません: {}",
                        texture_path.display()
                    );
                }

                let preview_path = preview_generator
                    .generate(&texture_path, &custom_model_data)
                    .context("プレビュー画像の生成に失敗しました")?;

                // Get environment variables for URL generation
                let pr_branch = std::env::var("PR_BRANCH")
                    .unwrap_or_else(|_| format!("custom-model/issue-{}", issue_number));

                let preview_url = PreviewGenerator::generate_url(
                    REPO_OWNER,
                    REPO_NAME,
                    &pr_branch,
                    &preview_path,
                );

                println!("✓ プレビュー画像の生成が完了しました");
                println!("\n=== 処理が正常に完了しました ===\n");

                Ok(ProcessResult {
                    preview_url: Some(preview_url),
                    custom_model_data,
                    added_materials: None,
                })
            }
            ParsedIssueData::Extend {
                materials,
                custom_model_data,
            } => {
                println!("  タイプ: Extend");
                println!("  追加するマテリアル: {}", materials.join(", "));
                println!("  カスタムモデルデータ: {}", custom_model_data);

                // Step 3: Extend materials
                println!("\n⚙️  マテリアルを拡張中...");
                let processor = Processor::new(custom_model_data.clone());
                processor
                    .extend_materials(&materials)
                    .context("マテリアルの拡張に失敗しました")?;
                println!("✓ マテリアルの拡張が完了しました");

                println!("\n=== 処理が正常に完了しました ===\n");

                Ok(ProcessResult {
                    preview_url: None,
                    custom_model_data,
                    added_materials: Some(materials),
                })
            }
        }
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

    /// Post success comment for Extend (no preview)
    pub fn post_extend_success(
        &self,
        issue_number: u64,
        pr_number: u64,
        materials: &[String],
    ) -> Result<()> {
        let materials_list = materials
            .iter()
            .map(|m| format!("- `{}`", m))
            .collect::<Vec<_>>()
            .join("\n");

        let comment = format!(
            r#"## ✅ マテリアルの拡張が完了しました！

**Pull Request:** #{}

### 追加されたマテリアル

{}

既存のカスタムモデルに上記のマテリアルが追加されました。PRをレビューしてマージしてください。"#,
            pr_number, materials_list
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
