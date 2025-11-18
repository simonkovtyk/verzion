use std::fs;

pub fn write_plain_file (path: &str, content: &str) {
  fs::write(path, content).expect("Failed to write file");
}
