use std::{env, fs};


pub fn visit() -> Result<(), String> {
    let current_dir = env::current_dir().expect("[can't get current location]");
    for entry in fs::read_dir(current_dir).expect("[ can't read file]") {
        let entry = entry.expect("[ can't read file]");
        let path = entry.path();
        let metadata = fs::metadata(&path).expect("[ can't get file metadata ]");
        let mut last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        let permission: &str;
        if metadata.permissions().readonly() {
            permission = "read";
        } else {
            permission = "write";
        }
        let day = last_modified / (3600*24);
        last_modified = last_modified % (3600*24);
        let hour = last_modified / 3600;
        last_modified = last_modified % 3600;
        let minute = last_modified / 60;

        println!(
            "{:20} {} {:2} Day {:2} Hour {:2} Minute {:10}",
            path.file_name().unwrap().to_str().unwrap(),
            permission,
            day,
            hour,
            minute,
            metadata.len(),
        );
    }
    Ok(())
}