mod comment;
mod reaction;

#[derive(Debug, clap::Parser)]
pub struct Runner {
    #[command(subcommand)]
    pub subcommand: RunnerSubcommand,
}

#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum RunnerSubcommand {
    Comment(comment::Comment),
    Reaction(reaction::Reaction),
}

impl super::Run for RunnerSubcommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            RunnerSubcommand::Comment(cmd) => cmd.run(),
            RunnerSubcommand::Reaction(cmd) => cmd.run(),
        }
    }
}

impl super::Run for Runner {
    fn run(&self) -> anyhow::Result<()> {
        self.subcommand.run()
    }
}
