use std::fs;

pub fn write_str_to_file (path: &str, content: &str) {
  fs::write(path, content).expect("Failed to write file");
}
