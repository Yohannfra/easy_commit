use inquire::InquireError;
use std::process;

pub fn handle_inquire_error(err: InquireError, msg: &str) -> ! {
    match err {
        InquireError::OperationCanceled => process::exit(0),
        InquireError::OperationInterrupted => process::exit(1),
        _ => panic!("{}", msg),
    }
}
