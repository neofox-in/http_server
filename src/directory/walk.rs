use std::{fs};
use std::path::PathBuf;

pub struct Directory {
    path: String,
    is_file: bool,
    filename: String,
    filesize: f32,
}


pub fn list_dir(dir_name: PathBuf) -> Vec<Directory> {
    let mut collection: Vec<Directory> = vec![];
    for entry in fs::read_dir(dir_name).expect("Dir Not Exits") {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        collection.push(Directory {
            path: path.to_str().unwrap().to_string(),
            is_file: metadata.file_type().is_file(),
            // is_dir: metadata.file_type().is_dir(),
            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
            filesize: (metadata.len() as f32) / 100.0,
        })
    }
    collection
}


pub fn visit(path: PathBuf) -> String {
    let result = list_dir(path);
    let mut table_body = "".to_string();
    for data in result {

        if data.is_file {
            table_body.push_str(&"<tr>".to_string());
            table_body.push_str(&format!("<td class='name'><a target='_blank' rel='noreferrer noopener' href='/file/{}'>{}</a></td>", data.path, data.filename));
        } else {
            table_body.push_str(&"<tr >".to_string());
            table_body.push_str(&format!("<td class='name folder'><a href='/{}'>{}</a></td>", data.path, data.filename));
        }
        table_body.push_str(&format!("<td class='path'>{}</td>", data.path));
        table_body.push_str(&format!("<td class='size'>{}</td>", data.filesize));
        table_body.push_str(&"</tr>".to_string());
    }
    table_body
}