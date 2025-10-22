mod post_failure;
mod process_issue;

#[derive(Debug, clap::Parser)]
pub struct Runner {
    #[command(subcommand)]
    pub subcommand: RunnerSubcommand,
}

#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum RunnerSubcommand {
    ProcessIssue(process_issue::ProcessIssue),
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
