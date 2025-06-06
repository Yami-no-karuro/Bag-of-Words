use std::time;
use std::time::{
    SystemTime,
    SystemTimeError
};

pub fn get_unix_timestamp() -> Result<u64, SystemTimeError> {
    let now: SystemTime = SystemTime::now();
    let duration = now.duration_since(time::UNIX_EPOCH)?;

    return Ok(
        duration.as_secs()
    );
}

