mod close;
mod comment;
mod generate_preview;
mod parse_issue;
mod post_extend_success;
mod post_failure;
mod post_success;
mod process_issue;
mod reaction;

#[derive(Debug, clap::Parser)]
pub struct Runner {
    #[command(subcommand)]
    pub subcommand: RunnerSubcommand,
}

#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum RunnerSubcommand {
    ProcessIssue(process_issue::ProcessIssue),
    ParseIssue(parse_issue::ParseIssue),
    PostSuccess(post_success::PostSuccess),
    PostExtendSuccess(post_extend_success::PostExtendSuccess),
    PostFailure(post_failure::PostFailure),
    GeneratePreview(generate_preview::GeneratePreview),
    Comment(comment::Comment),
    Reaction(reaction::Reaction),
    Close(close::Close),
}

impl super::Run for RunnerSubcommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            RunnerSubcommand::ProcessIssue(cmd) => cmd.run(),
            RunnerSubcommand::ParseIssue(cmd) => cmd.run(),
            RunnerSubcommand::PostSuccess(cmd) => cmd.run(),
            RunnerSubcommand::PostExtendSuccess(cmd) => cmd.run(),
            RunnerSubcommand::PostFailure(cmd) => cmd.run(),
            RunnerSubcommand::GeneratePreview(cmd) => cmd.run(),
            RunnerSubcommand::Comment(cmd) => cmd.run(),
            RunnerSubcommand::Reaction(cmd) => cmd.run(),
            RunnerSubcommand::Close(cmd) => cmd.run(),
        }
    }
}

impl super::Run for Runner {
    fn run(&self) -> anyhow::Result<()> {
        self.subcommand.run()
    }
}
