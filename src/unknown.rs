use crate::utils::{
    print_help,
    exit
};

pub fn handle_unknown(command: &str) {
    eprintln!("Error: unknown command \"{}\".", command);
    print_help();
    exit(1);
}

