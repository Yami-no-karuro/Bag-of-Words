mod proc;
mod time;
mod fs;

pub use proc::exit;
pub use time::get_unix_timestamp;
pub use fs::{
    read_content,
    dump_content
};

