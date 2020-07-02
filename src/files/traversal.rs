use std::env::current_dir;
use std::fs::{read_dir, ReadDir, DirEntry};


fn visit_dir(_dir_name: String) -> Result<ReadDir, String> {
    let mut _current_dir = current_dir().expect("[can't get current location]");
    if !_dir_name.is_empty() {
        _current_dir.push(_dir_name);
    }
    if !_current_dir.is_dir() {
        _current_dir.clear();
        _current_dir = current_dir().expect("[can't get current location]");
    }
    Ok(read_dir(_current_dir).expect("[can't read file]"))
}


pub fn visit(_dir_name: String) -> Result<(Vec<DirEntry>, Vec<DirEntry>, Vec<DirEntry>), String> {
    let mut folders: Vec<DirEntry> = Vec::new();
    let mut files: Vec<DirEntry> = Vec::new();
    let mut error: Vec<DirEntry> = Vec::new();
    for dir in visit_dir(_dir_name)? {
        match dir {
            Ok(dir_entry) => match dir_entry.metadata() {
                Ok(dir_metadata) => {
                    if dir_metadata.is_dir() {
                        folders.push(dir_entry);
                    } else {
                        files.push(dir_entry);
                    }
                }
                _ => error.push(dir_entry)
            }
            _ => error.push(dir.unwrap())
        }
    }
    Ok((folders, files, error))
}