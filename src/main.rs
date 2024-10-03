use inquire::{Select, Text};
use std::process;
use std::process::Command;

use std::io::{self, Write};

mod commit_type;

use commit_type::CommitType;

#[derive(Debug)]
struct CommitData {
    commit_type: CommitType,
    message: String,
    scope: Option<String>,
}

fn main() {
    let mut commit_data = CommitData {
        commit_type: CommitType::Fix,
        scope: None,
        message: "".to_string(),
    };

    // Commit type
    commit_data.commit_type = match Select::new("Type:", CommitType::as_str_vec()).prompt() {
        Ok(commit_type) => CommitType::new(commit_type),
        Err(_) => {
            panic!("An error happened when asking for your commit_type, try again later.");
        }
    };

    // Scope
    commit_data.scope = match Text::new("Scope (Empty to skip):").prompt() {
        Ok(scope) => {
            if scope.is_empty() {
                None
            } else {
                Some(scope)
            }
        }
        Err(_) => {
            panic!("An error happened when asking for your commit_message, try again later.")
        }
    };

    // Message
    commit_data.message = match Text::new("Message:").prompt() {
        Ok(message) => message,
        Err(_) => {
            panic!("An error happened when asking for your commit_message, try again later.")
        }
    };

    println!("{:?}", commit_data);

    let full_message = format!(
        "{}{}: {}",
        commit_data.commit_type.as_str(),
        if Option::is_some(&commit_data.scope) {
            format!("({})", commit_data.scope.unwrap())
        } else {
            "".to_string()
        },
        commit_data.message
    );

    println!("{}", full_message);

    let output = Command::new("git")
        .arg("commit")
        //.arg("--dry-run")
        .arg("-m")
        .arg(full_message)
        .output()
        .expect("Failed to execute commit");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    process::exit(output.status.code().unwrap());
}
