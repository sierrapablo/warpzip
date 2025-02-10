use std::path::Path;

pub fn file_exists(file: &str) -> bool {
    Path::new(file).exists()
}
