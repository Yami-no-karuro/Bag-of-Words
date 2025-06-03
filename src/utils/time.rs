use std::time;
use std::time::SystemTime;

pub fn get_unix_timestamp() -> u64 {
    let now: SystemTime = SystemTime::now();
    let duration = now.duration_since(time::UNIX_EPOCH)
        .unwrap();

    return duration.as_secs();
}

