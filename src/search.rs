use std::fs;
use std::fs::DirEntry;
use std::path::{
    Path, 
    PathBuf
};

use crate::utils::proc::exit;
use crate::utils::fs::read_content;

fn load_source(entry: DirEntry) {
    let path_buf: PathBuf = entry.path();
    let path: &Path = path_buf.as_path();
    let path_str: &str = path.to_str()
        .unwrap();

    if let Some(extension) = path.extension() {
        if extension == "mdl" {
            let source: String = read_content(path_str).unwrap();
            dbg!(source.lines().next());
        }
    }
}

pub fn handle_search(args: &[String]) {
    let query: &str = &args[0];
    if let Ok(entries) = fs::read_dir("models") {
        for entry in entries {
            let dir_entry: DirEntry = entry.unwrap();
            load_source(dir_entry);
        }
    } else {
        eprintln!("Error: unable to read the models directory.");
        exit(1);
    }

    dbg!(query);
    exit(0);
}

