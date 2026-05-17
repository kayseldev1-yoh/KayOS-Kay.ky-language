use std::fs;
pub fn read_file(path: &str) -> String { fs::read_to_string(path).unwrap_or_default() }
pub fn write_file(path: &str, content: &str) { fs::write(path, content).unwrap(); }
pub fn exists(path: &str) -> bool { fs::metadata(path).is_ok() }
