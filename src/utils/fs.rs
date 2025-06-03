use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{
    Read,
    Write
};

pub fn read_content(path: &Path) -> Result<String, io::Error> {
    let mut content: String = String::new();
    let mut file: File = File::open(path)?;
    file.read_to_string(&mut content)?;

    return Ok(content);
}

pub fn dump_content(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file: File = File::create(path)?;
    file.write_all(content.as_bytes())?;

    return Ok(());
}

