use std::process;
use std::time;
use std::time::SystemTime;

pub fn get_unix_timestamp() -> u64 {
    let now: SystemTime = SystemTime::now();
    let duration = now.duration_since(time::UNIX_EPOCH)
        .unwrap();

    return duration.as_secs();
}

pub fn exit(status: i32) {
    process::exit(status);
}

pub fn print_help() {
    println!("Usage: ");
    println!("- index <path>");
}

