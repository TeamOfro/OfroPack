#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum IssueType {
    Model,
    Model3d,
    Extend,
}

impl IssueType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            IssueType::Model => "model",
            IssueType::Model3d => "model3d",
            IssueType::Extend => "extend",
        }
    }
}
