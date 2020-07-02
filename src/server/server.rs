use nickel::{Nickel, HttpRouter};
use regex::Regex;
use super::super::files::traversal;
use std::fs::DirEntry;
use std::fs::read_dir;


fn handle_files(_files: Vec<DirEntry>, mut _path: String) -> String {
    let mut response = String::from("<table style=\"width:100%;margin:40px\">");
    if _path.len() < 2 {
        _path = String::from("");
    } else {
        _path.push_str("/");
    }
    if !_files.is_empty() {
        for _file in _files {
            response.push_str(&format!("<tr><td><a href=\"/{}{}\">{}</a></td><td>{}</td><td>{}</td></tr>",
                                       _path,
                                       _file.path().file_name().unwrap().to_str().unwrap(),
                                       _file.path().file_name().unwrap().to_str().unwrap(),
                                       _file.metadata().unwrap().created().unwrap().elapsed().unwrap().as_secs(),
                                       _file.metadata().unwrap().len()
            ))
        }
    }
    response.push_str("</table>");
    response
}


fn handle_dirs(_files: Vec<DirEntry>, mut _path: String) -> String {
    let mut response = String::from("<table style=\"width:100%;margin:40px\">");
    if _path.len() < 2 {
        _path = String::from("");
    } else {
        _path.push_str("/");
    }
    if !_files.is_empty() {
        for _file in _files {
            response.push_str(&format!("<tr><td><a href=\"/{}{}\">{}</a></td><td>{}</td><td>{}</td></tr>",
                                       _path,
                                       _file.path().file_name().unwrap().to_str().unwrap(),
                                       _file.path().file_name().unwrap().to_str().unwrap(),
                                       _file.metadata().unwrap().created().unwrap().elapsed().unwrap().as_secs(),
                                       read_dir(_file.path()).unwrap().count()
            ))
        }
    }
    response.push_str("</table>");
    response
}


fn handle_request(_path: String) -> String {
    let (_dirs, _files, _errors) = traversal::visit(_path.clone()).unwrap();
    // const table: String = String::from("");
    let mut _dir_response: String = handle_dirs(_dirs, _path.clone());
    let mut _files_response: String = handle_files(_files, _path.clone());

    format!("{}{}", _dir_response, _files_response)
}

pub fn serve() {
    let mut server = Nickel::new();
    let path_regex = Regex::new(r"/(?P<path>[\d\D\s\S\w\W]*)$").unwrap();
    server.get(path_regex, middleware! { |request|
       let _path = request.param("path").unwrap();
       handle_request(String::from(_path))
    });

    server.listen("127.0.0.1:6767").unwrap();
}