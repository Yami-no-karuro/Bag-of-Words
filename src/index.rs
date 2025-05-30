use std::fs;
use std::fs::File;
use std::ffi::OsStr;
use std::path::{
    Path, 
    PathBuf
};
use std::io::{
    Read, 
    Write
};

use crate::bow::BoW;
use crate::utils::exit;

fn read_content(path: &Path) -> String {
    let mut content: String = String::new();
    let mut file: File = File::open(path).unwrap();
    file.read_to_string(&mut content)
        .unwrap();

    return content;
}

fn write_content(filename: &str, content: &str) {
    let output_path: String = format!("models/{}", filename);
    let mut file: File = File::create(output_path).unwrap();
    file.write_all(content.as_bytes())
        .unwrap();
}

fn process_source(path: &Path) {
    let content: String = read_content(path);
    let bag: BoW = BoW::build(content);
    let dump: String = bag.to_string();

    let name: &str = path.file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    write_content(name, &dump);
}

pub fn handle_index(args: &[String]) {
    let source: &str = &args[0];
    let source_path: PathBuf = PathBuf::from(source);
    if !source_path.is_dir() {
        eprintln!("Error: the provided source (\"{}\") is not a directory", source);
        exit(1);
    }

    if let Ok(entries) = fs::read_dir(source) {
        for entry in entries {
            let path: PathBuf = entry.unwrap()
                .path();
            
            process_source(&path);
        }
    } else {
        eprintln!("Error: unable to read the source (\"{}\") directory.", source);
        exit(1);
    }

    exit(0);
}

