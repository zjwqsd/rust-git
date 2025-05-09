use std::fs;
use std::path::{Path, PathBuf};

/// 递归列出所有文件
pub fn list_files(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                result.extend(list_files(&path));
            } else if path.is_file() {
                result.push(path);
            }
        }
    }
    result
}
