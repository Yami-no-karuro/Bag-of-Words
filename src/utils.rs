use std::process;

pub fn exit(status: i32) {
    process::exit(status);
}

pub fn print_help() {
    println!("Usage: ");
    println!("- index <path>");
}

