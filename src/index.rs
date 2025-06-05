use std::fs;
use std::fs::DirEntry;
use std::ffi::OsString;
use std::path::{
    Path, 
    PathBuf
};

use crate::bow::BoW;
use crate::utils::proc::exit;
use crate::utils::time::get_unix_timestamp;
use crate::utils::fs::{
    read_content,
    dump_content
};

fn get_metadata(path: &str, bow: &BoW) -> String {
    let time: u64 = get_unix_timestamp().unwrap();
    let size: usize = bow.get_vocabulary_size();
    let occurrences: usize = bow.get_total_occurences();

    let metadata: String = format!(
        "{},{},{},{}",
        path,
        time,
        size,
        occurrences
    );

    return metadata;
}

fn dump_source(entry: DirEntry) {
    let name: OsString = entry.file_name();
    let name_str: &str = name.to_str()
        .unwrap();

    let path_buf: PathBuf = entry.path();
    let path: &Path = path_buf.as_path();
    let path_str: &str = path.to_str()
        .unwrap();

    let source: String = read_content(path_str).unwrap();
    let bag: BoW = BoW::build(source);
    let metadata: String = get_metadata(path_str, &bag);

    let dump: String = bag.to_string();
    let output: String = format!("{}\n[{}]", metadata, dump);
    let output_path: String = format!("models/{}.mdl", name_str);

    dump_content(
        &output_path, 
        &output
    ).unwrap();
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
            let dir_entry: DirEntry = entry.unwrap();
            dump_source(dir_entry);
        }
    } else {
        eprintln!("Error: unable to read the source (\"{}\") directory.", source);
        exit(1);
    }

    exit(0);
}

