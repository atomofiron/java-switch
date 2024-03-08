use crate::operation::Operation;

pub struct TrayOption {
    label: String,
    pub operation: Operation,
}

impl TrayOption {

    pub fn new(label: String, operation: Operation) -> TrayOption {
        TrayOption { label, operation }
    }

    pub fn label(&self, applied: bool) -> String {
        let marker = match self.operation {
            Operation::Apply(_) if applied => "-> ",
            _ => "",
        };
        return format!("{marker}{}", self.label.clone());
    }

    pub fn has_path(&self, value: &String) -> bool {
        match &self.operation {
            Operation::Apply(path) => path == value,
            Operation::Refresh => false,
        }
    }
}

impl Clone for TrayOption {
    fn clone(&self) -> Self {
        TrayOption {
            label: self.label.clone(),
            operation: self.operation.clone(),
        }
    }
}
