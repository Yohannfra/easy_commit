use inquire::{InquireError, Select, Text};

struct Commit {
    r#type: String,
    message: String,
}

fn main() {
    let commit = Commit {
        r#type: "".to_string(),
        message: "".to_string(),
    };

    let commit_type_options: Vec<&str> = vec!["feat", "fix", "refactor"];
    let commit_type: Result<&str, InquireError> =
        Select::new("Type ?", commit_type_options).prompt();
    match commit_type {
        Ok(commit_type) => println!("{}", commit_type),
        Err(_) => println!("An error happened when asking for your commit_type, try again later."),
    }

    let commit_message = Text::new("Message ?").prompt();

    match commit_message {
        Ok(commit_message) => println!("{}", commit_message),
        Err(_) => {
            println!("An error happened when asking for your commit_message, try again later.")
        }
    }
}
