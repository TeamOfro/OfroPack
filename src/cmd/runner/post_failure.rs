use anyhow::Result;

use crate::{cmd::Run, pipeline::runner::process_issue::IssueProcessor};

/// ❌ 失敗時にIssueにコメントを投稿
///
/// ワークフロー失敗時にエラーメッセージをIssueに投稿します。
#[derive(clap::Parser, Debug)]
#[command(
    about = "失敗時にIssueにコメントを投稿",
    long_about = "ワークフロー失敗時にエラーメッセージをIssueに投稿します。\n\n\
                  エラー内容とワークフローURLを含むコメントを自動生成します。"
)]
pub struct PostFailure {
    /// Issue番号
    #[arg(long, value_name = "NUMBER", help = "コメントを投稿するIssue番号")]
    issue_number: u64,

    /// エラーメッセージ
    #[arg(long, value_name = "MESSAGE", help = "表示するエラーメッセージ")]
    error_message: String,

    /// ワークフローURL
    #[arg(
        long,
        value_name = "URL",
        help = "失敗したワークフローのURL（ログ確認用）"
    )]
    workflow_url: String,
}

impl Run for PostFailure {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        processor.post_failure(self.issue_number, &self.error_message, &self.workflow_url)?;
        Ok(())
    }
}
