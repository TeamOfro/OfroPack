use anyhow::Result;

use crate::{cmd::Run, pipeline::runner::process_issue::IssueProcessor, types::IssueType};

/// 📝 Issueを処理してPRを作成
///
/// GitHub IssueからカスタムモデルデータやPRを自動生成します。
#[derive(clap::Parser, Debug)]
#[command(
    about = "Issueを処理してPRを作成",
    long_about = "GitHub IssueからカスタムモデルデータやPRを自動生成します。\n\n\
                  画像のダウンロード、バリデーション、モデル生成、PR作成、\n\
                  コメント投稿までを自動で実行します。"
)]
pub struct ProcessIssue {
    /// Issue番号
    #[arg(long, value_name = "NUMBER", help = "処理対象のIssue番号")]
    issue_number: u64,

    /// Issue種別
    #[arg(
        long,
        value_name = "TYPE",
        help = "Issue種別（model, model3d, extend）"
    )]
    issue_type: IssueType,

    /// Issue本文
    #[arg(long, value_name = "BODY", help = "Issueの本文（パース用）")]
    body: String,

    /// 作成者ユーザー名
    #[arg(long, value_name = "USERNAME", help = "Issue作成者のユーザー名")]
    actor: String,

    /// 作成者メールアドレス
    #[arg(
        long,
        value_name = "EMAIL",
        help = "Git commitに使用するメールアドレス"
    )]
    actor_email: String,
}

impl Run for ProcessIssue {
    fn run(&self) -> Result<()> {
        let processor = IssueProcessor::new()?;
        processor.process(
            self.issue_number,
            self.issue_type,
            &self.body,
            &self.actor,
            &self.actor_email,
        )?;
        Ok(())
    }
}
