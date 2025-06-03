use crate::utils::proc::exit;

pub fn handle_unknown(command: &str) {
    eprintln!("Error: unknown command \"{}\".", command);
    exit(1);
}

