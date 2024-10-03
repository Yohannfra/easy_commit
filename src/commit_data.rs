use crate::CommitType;

#[derive(Debug)]
pub struct CommitData {
    pub commit_type: CommitType,
    pub message: String,
    pub scope: Option<String>,
    pub no_verify: bool,
}

impl CommitData {
    pub fn new() -> Self {
        Self {
            commit_type: CommitType::Fix,
            scope: None,
            message: "".to_string(),
            no_verify: false,
        }
    }

    pub fn create_full_message(&self) -> String {
        format!(
            "{}{}: {}",
            self.commit_type.as_str(),
            if Option::is_some(&self.scope) {
                format!("({})", self.scope.as_ref().unwrap())
            } else {
                "".to_string()
            },
            self.message
        )
    }

    pub fn get_commit_command_as_str(&self) -> String {
        let full_message = self.create_full_message();

        format!(
            "git commit -m \"{}\" {}",
            full_message,
            if self.no_verify { "--no-verify" } else { "" }
        )
    }
}
