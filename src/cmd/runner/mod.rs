mod post_failure;
mod process_issue;

/// 🤖 GitHub Actions用ランナー
///
/// CI/CD環境でIssue処理などを実行します。
#[derive(Debug, clap::Parser)]
#[command(
    about = "GitHub Actions用ランナー",
    long_about = "CI/CD環境でIssue処理などを実行します。\n\n\
                  通常、このコマンドは手動で使用することはありません。"
)]
pub struct Runner {
    #[command(subcommand)]
    pub subcommand: RunnerSubcommand,
}

/// サブコマンド
#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum RunnerSubcommand {
    /// 📝 Issueを処理してPRを作成
    ///
    /// GitHub IssueからカスタムモデルデータやPRを自動生成します。
    ProcessIssue(process_issue::ProcessIssue),

    /// ❌ 失敗時にIssueにコメントを投稿
    ///
    /// ワークフロー失敗時にエラーメッセージをIssueに投稿します。
    PostFailure(post_failure::PostFailure),
}

impl super::Run for RunnerSubcommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::ProcessIssue(cmd) => cmd.run(),
            Self::PostFailure(cmd) => cmd.run(),
        }
    }
}

impl super::Run for Runner {
    fn run(&self) -> anyhow::Result<()> {
        self.subcommand.run()
    }
}
