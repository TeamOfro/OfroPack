mod model;
mod model3d;

#[derive(clap::Parser, Debug)]
pub struct Add {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
#[command(version, about)]
pub enum Subcommand {
    Model(model::Model),
    Model3D(model3d::Model3D),
}

impl super::Run for Subcommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Subcommand::Model(cmd) => cmd.run(),
            Subcommand::Model3D(cmd) => cmd.run(),
        }
    }
}

impl super::Run for Add {
    fn run(&self) -> anyhow::Result<()> {
        self.subcommand.run()
    }
}
