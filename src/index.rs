use std::fs;
use std::ffi::OsStr;
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

fn process_source(path: &Path) {
    let source: String = read_content(path).unwrap();
    let name: &str = path.file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    let bag: BoW = BoW::build(source);
    let dump: String = bag.to_string();
    let metadata: String = get_metadata(&path, &bag);
    let output: String = format!("{}\n[{}]", metadata, dump);

    let o_path: String = format!("models/{}.mdl", name);
    dump_content(&o_path, &output).unwrap();
}

fn get_metadata(path: &Path, bow: &BoW) -> String {
    let metadata: String = format!(
        "{},{},{},{}",
        path.display(),
        get_unix_timestamp(),
        bow.get_vocabulary_size(),
        bow.get_total_occurences()
    );

    return metadata;
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

