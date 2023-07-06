use std::fs;

pub fn get_file_contents(file_path: String) -> Option<String> {
    let content = fs::read_to_string(file_path);
    match content {
        Ok(content_str) => Some(content_str),
        Err(e) => None,
    }
}
