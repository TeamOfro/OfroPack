use anyhow::{Context, Result};
use tempfile::tempdir;

use crate::{
    cmd::{Run, add, extend},
    config::{REPO_NAME, REPO_OWNER},
    pipeline::{
        github_client::GitHubClient,
        image_downloader::ImageDownloader,
        pr_creator::PrCreator,
        preview_generator::PreviewGenerator,
        runner::issue_parser::{IssueParser, ParsedIssue},
    },
    types::{GithubReaction, IssueType},
};

pub struct IssueProcessor {
    github_client: GitHubClient,
    image_downloader: ImageDownloader,
    pr_creator: PrCreator,
}

impl IssueProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            github_client: GitHubClient::from_env()?,
            image_downloader: ImageDownloader::new()?,
            pr_creator: PrCreator::new()?,
        })
    }

    fn format_materials(materials: &[String]) -> String {
        materials
            .iter()
            .map(|m| format!("`{}`", m))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn format_pr_branch(prefix: &str, issue_number: u64) -> String {
        std::env::var("PR_BRANCH")
            .unwrap_or_else(|_| format!("{}-{}/issue-{}", prefix, prefix, issue_number))
    }

    fn add_success_reaction(&self, issue_number: u64) -> Result<()> {
        self.github_client
            .react_issue(issue_number, GithubReaction::ThumbsUp)
            .context("Reactionの追加に失敗しました")
    }

    pub fn process(
        &self,
        issue_number: u64,
        issue_type: IssueType,
        issue_body: &str,
        actor: &str,
        actor_email: &str,
    ) -> Result<()> {
        println!("\n🚀 === Issue #{}の処理を開始 ===\n", issue_number);
        println!("📋 Issue種別: {:?}", issue_type);
        println!("👤 作成者: {}", actor);

        // 1. Rocket reaction
        println!("\n🚀 Rocketリアクションを追加中...");
        self.github_client
            .react_issue(issue_number, GithubReaction::Rocket)
            .context("❌ Reactionの追加に失敗しました")?;
        println!("✅ Rocketリアクションを追加しました");

        // 2. Parse issue
        println!("\n📝 Issueを解析中...");
        let parsed =
            IssueParser::parse(issue_body, issue_type).context("❌ Issueの解析に失敗しました")?;
        println!("✅ Issueの解析が完了しました");

        // 3. Process based on issue type
        match parsed {
            ParsedIssue::Model {
                materials,
                custom_model_data,
                image_url,
                animation,
                parent,
            } => {
                println!("  タイプ: Model");
                println!("  マテリアル: {}", materials.join(", "));
                println!("  カスタムモデルデータ: {}", custom_model_data);
                println!("  画像URL: {}", image_url);
                println!("  モデル親: {}", parent.as_str());
                if let Some(anim) = &animation {
                    println!("  Frametime: {}", anim.animation.frametime);
                }

                let dir = tempdir()?;
                let image_path = dir.path().join(format!("{}.png", custom_model_data));

                self.image_downloader
                    .download(&image_url, &image_path)
                    .context("画像のダウンロードに失敗しました")?;

                let frametime = animation.as_ref().map(|a| a.animation.frametime);

                let add_cmd = add::model::Model::new(
                    materials.clone(),
                    Some(custom_model_data.clone()),
                    frametime,
                    image_path,
                    parent,
                );
                add_cmd.run()?;

                println!("\n🖼️  プレビュー画像を生成中...");
                let texture_path = crate::paths::Paths::texture_path(&custom_model_data);
                let preview_path = PreviewGenerator::generate(&texture_path, &custom_model_data)
                    .context("プレビュー画像の生成に失敗しました")?;

                let pr_branch = Self::format_pr_branch("model", issue_number);

                let preview_url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}",
                    REPO_OWNER,
                    REPO_NAME,
                    pr_branch,
                    preview_path.to_string_lossy()
                );

                println!("✓ プレビュー画像の生成が完了しました");

                // 4. Create PR
                let pr_title = format!("✨ 2Dモデルを追加: {}", custom_model_data);
                let animation_line = animation
                    .as_ref()
                    .map(|a| {
                        format!(
                            "- **アニメーション:** frametime = {}",
                            a.animation.frametime
                        )
                    })
                    .unwrap_or_default();

                let pr_body = format!(
                    r"## 📦 2Dカスタムモデルの追加

**Issue:** #{}

### 追加内容

- **カスタムモデルデータ:** `{}`
- **マテリアル:** {}
- **モデル親:** `{}`
{}

### プレビュー（256×256、ピクセルパーフェクト）

![Custom Model Preview]({})

---

このPRは自動生成されました。",
                    issue_number,
                    custom_model_data,
                    Self::format_materials(&materials),
                    parent.as_str(),
                    animation_line,
                    preview_url
                );

                let pr_number = self.pr_creator.create_pr(
                    &pr_branch,
                    &pr_title,
                    &pr_body,
                    actor,
                    actor_email,
                )?;

                // 5. Post success comment
                let comment = format!(
                    r"## ✅ 2Dカスタムモデルの処理が完了しました！

**Pull Request:** #{}

### プレビュー（256×256、ピクセルパーフェクト）

![Custom Model Preview]({})

このカスタムモデルをリソースパックに追加するため、PRをレビューしてマージしてください。",
                    pr_number, preview_url
                );

                self.github_client
                    .comment_issue(issue_number, &comment)
                    .context("成功コメントの投稿に失敗しました")?;

                // 6. ThumbsUp reaction
                self.add_success_reaction(issue_number)?;

                println!("\n=== 処理が正常に完了しました ===\n");
                Ok(())
            }
            ParsedIssue::Model3d {
                materials,
                custom_model_data,
                model_json_url,
                layer_image_urls,
            } => {
                println!("  タイプ: Model3d");
                println!("  マテリアル: {}", materials.join(", "));
                println!("  カスタムモデルデータ: {}", custom_model_data);
                println!("  モデルJSON URL: {}", model_json_url);
                println!("  レイヤー画像 URL: {}", layer_image_urls.join("\n"));

                let dir = tempdir()?;
                let model_json_path = dir.path().join(format!("{}.json", custom_model_data));
                self.image_downloader
                    .download(&model_json_url, &model_json_path)
                    .context("モデルJSONのダウンロードに失敗しました")?;

                let mut layer_image_paths = Vec::new();
                for (i, url) in layer_image_urls.iter().enumerate() {
                    let image_path = dir.path().join(format!("{}_{}.png", custom_model_data, i));
                    self.image_downloader
                        .download(url, &image_path)
                        .context(format!("レイヤー画像 {} のダウンロードに失敗しました", i))?;
                    layer_image_paths.push(image_path);
                }

                let add_cmd = add::model3d::Model3D::new(
                    materials.clone(),
                    custom_model_data.clone(),
                    model_json_path,
                    layer_image_paths,
                );
                add_cmd.run()?;

                // 4. Create PR
                let pr_branch = Self::format_pr_branch("model3d", issue_number);

                let pr_title = format!("✨ 3Dモデルを追加: {}", custom_model_data);
                let pr_body = format!(
                    r"## 📦 3Dカスタムモデルの追加

**Issue:** #{}

### 追加内容

- **カスタムモデルデータ:** `{}`
- **マテリアル:** {}
- **レイヤー数:** {}

---

このPRは自動生成されました。",
                    issue_number,
                    custom_model_data,
                    Self::format_materials(&materials),
                    layer_image_urls.len()
                );

                let pr_number = self.pr_creator.create_pr(
                    &pr_branch,
                    &pr_title,
                    &pr_body,
                    actor,
                    actor_email,
                )?;

                // 5. Post success comment
                let comment = format!(
                    r"## ✅ 3Dカスタムモデルの処理が完了しました！

**Pull Request:** #{}

3Dモデルがリソースパックに追加されました。PRをレビューしてマージしてください。",
                    pr_number
                );

                self.github_client
                    .comment_issue(issue_number, &comment)
                    .context("成功コメントの投稿に失敗しました")?;

                // 6. ThumbsUp reaction
                self.add_success_reaction(issue_number)?;

                println!("\n=== 処理が正常に完了しました ===\n");
                Ok(())
            }
            ParsedIssue::Extend {
                materials,
                custom_model_data,
            } => {
                println!("  タイプ: Extend");
                println!("  追加するマテリアル: {}", materials.join(", "));
                println!("  カスタムモデルデータ: {}", custom_model_data);

                let extend_cmd = extend::Extend {
                    materials: materials.clone(),
                    custom_model_data: custom_model_data.clone(),
                };
                extend_cmd.run()?;

                // 4. Create PR
                let pr_branch = Self::format_pr_branch("extend", issue_number);

                let pr_title = format!("✨ マテリアルを拡張: {}", custom_model_data);
                let materials_list = materials
                    .iter()
                    .map(|m| format!("  - `{}`", m))
                    .collect::<Vec<_>>()
                    .join("\n");

                let pr_body = format!(
                    r"## 📦 マテリアルの拡張

**Issue:** #{}

### 追加内容

- **カスタムモデルデータ:** `{}`
- **追加されたマテリアル:**
{}

---

このPRは自動生成されました。",
                    issue_number, custom_model_data, materials_list
                );

                let pr_number = self.pr_creator.create_pr(
                    &pr_branch,
                    &pr_title,
                    &pr_body,
                    actor,
                    actor_email,
                )?;

                // 5. Post success comment
                let materials_list_comment = materials
                    .iter()
                    .map(|m| format!("- `{}`", m))
                    .collect::<Vec<_>>()
                    .join("\n");

                let comment = format!(
                    r"## ✅ マテリアルの拡張が完了しました！

**Pull Request:** #{}

### 追加されたマテリアル

{}

既存のカスタムモデルに上記のマテリアルが追加されました。PRをレビューしてマージしてください。",
                    pr_number, materials_list_comment
                );

                self.github_client
                    .comment_issue(issue_number, &comment)
                    .context("成功コメントの投稿に失敗しました")?;

                // 6. ThumbsUp reaction
                self.add_success_reaction(issue_number)?;

                println!("\n=== 処理が正常に完了しました ===\n");
                Ok(())
            }
        }
    }

    pub fn post_failure(
        &self,
        issue_number: u64,
        error_message: &str,
        workflow_url: &str,
    ) -> Result<()> {
        let comment = format!(
            r"## ❌ カスタムモデルの処理に失敗しました

ワークフローでエラーが発生しました。詳細は[ワークフローログ]({})を確認してください。

### エラー内容

```
{}
```

### 次のステップ

1. ログを確認してエラーの原因を特定する
2. 入力内容を修正する
3. **新しいIssueを作成する**（正しい情報を入力）

⚠️ **注意:** このIssueを編集しても再実行されません。新しいIssueを作成してください。",
            workflow_url, error_message
        );

        self.github_client
            .comment_issue(issue_number, &comment)
            .context("エラーコメントの投稿に失敗しました")?;

        self.github_client
            .react_issue(issue_number, crate::types::GithubReaction::ThumbsDown)
            .context("Reactionの追加に失敗しました")?;

        self.github_client
            .close_issue(issue_number)
            .context("Issueのクローズに失敗しました")?;

        Ok(())
    }
}
