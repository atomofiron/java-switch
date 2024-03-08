
pub enum Operation {
    Apply(String), Refresh
}

impl Clone for Operation {
    fn clone(&self) -> Self {
        match self {
            Operation::Apply(path) => Operation::Apply(path.clone()),
            Operation::Refresh => Operation::Refresh,
        }
    }
}

