#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum IssueType {
    Model,
    Model3d,
    Extend,
}
