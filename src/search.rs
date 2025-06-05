use std::fs;
use std::fs::DirEntry;
use std::path::{
    Path, 
    PathBuf
};

use crate::bow::BoW;
use crate::utils::proc::exit;
use crate::utils::fs::read_content;

fn load_source(entry: DirEntry) {
    let path_buf: PathBuf = entry.path();
    let path: &Path = path_buf.as_path();
    let path_str: &str = path.to_str()
        .unwrap();

    if let Some(extension) = path.extension() {
        if extension != "mdl" {
            return;
        }
    } else {
        return;
    }

    let source: String = read_content(path_str).unwrap();
    let mut lines = source.lines();
    let metadata: &str = lines.next()
        .unwrap();

    let content: &str = lines.next()
        .unwrap();

    let bag: BoW = BoW::from_serialized(content);
    dbg!(metadata);
    dbg!(bag);
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

    exit(0);
}

