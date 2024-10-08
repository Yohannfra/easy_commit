use clap::Parser;
use inquire::{Select, Text};
use std::process;
use std::process::{Command, Stdio};

mod commit_data;
mod commit_type;
mod config;
mod error_handler;

use commit_data::CommitData;
use commit_type::CommitType;
use config::Config;
use error_handler::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Skip hooks
    #[arg(long, action)]
    no_verify: bool,

    /// man git-commit --try-run (Don't actually create the commit)
    #[arg(long, action)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    //println!("{:?}", args);
    //process::exit(0);

    let config: Config = Config::new(std::path::PathBuf::from(".easy-commit.toml")).unwrap();

    std::process::exit(0);

    let mut commit_data = CommitData::new();

    commit_data.no_verify = args.no_verify;

    // Commit type
    commit_data.commit_type = match Select::new("Type:", CommitType::as_str_vec()).prompt() {
        Ok(commit_type) => CommitType::new(commit_type),
        Err(e) => handle_inquire_error(
            e,
            "An error happened when asking for your commit_type, try again later.",
        ),
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
        Err(e) => handle_inquire_error(
            e,
            "An error happened when asking for your commit_message, try again later.",
        ),
    };

    // Message
    commit_data.message = match Text::new("Message:").prompt() {
        Ok(message) => message,
        Err(e) => handle_inquire_error(
            e,
            "An error happened when asking for your commit_message, try again later.",
        ),
    };

    println!("{}", commit_data.get_commit_command_as_str());

    let output = Command::new("git")
        .arg("commit")
        .args(if args.dry_run {
            vec!["--dry-run"]
        } else {
            vec![]
        })
        .arg("-m")
        .arg(commit_data.create_full_message())
        .args(if args.no_verify {
            vec!["--no-verify"]
        } else {
            vec![]
        })
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute commit");

    process::exit(output.status.code().unwrap());
}
