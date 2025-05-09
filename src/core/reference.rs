use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// 设置 HEAD 指向新的分支
pub fn set_head(ref_path: &str, repo_path: &Path) -> io::Result<()> {
    fs::write(repo_path.join("HEAD"), format!("ref: {}\n", ref_path))
}

/// 获取 HEAD 当前指向的引用路径（如 refs/heads/main）
pub fn get_head_ref(repo_path: &Path) -> io::Result<PathBuf> {
    let head_path = repo_path.join("HEAD");
    let content = fs::read_to_string(&head_path)?;
    if content.starts_with("ref: ") {
        let rel_ref = content[5..].trim(); // e.g. "refs/heads/main"
        Ok(repo_path.join(rel_ref))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "HEAD 非符号引用"))
    }
}

/// 获取当前 HEAD 指向的分支名，如 "main"
pub fn get_current_branch_name(repo_path: &Path) -> Option<String> {
    let head_path = repo_path.join("HEAD");
    let content = fs::read_to_string(head_path).ok()?;
    if content.starts_with("ref: ") {
        let rel = content.trim().strip_prefix("ref: refs/heads/")?;
        Some(rel.to_string())
    } else {
        None
    }
}
