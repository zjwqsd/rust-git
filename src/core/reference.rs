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

pub fn validate_branch_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("分支名不能为空".into());
    }

    if name == "." || name == ".." {
        return Err("分支名不能为 '.' 或 '..'".into());
    }

    if name.starts_with('/') || name.ends_with('/') {
        return Err("分支名不能以 '/' 开头或结尾".into());
    }

    if name.contains("//") {
        return Err("分支名不能包含连续的 '/'".into());
    }

    if name.contains("..") {
        return Err("分支名不能包含 '..'".into());
    }

    if name.contains(['~', '^', ':', '?', '*', '[', '\\', ' ']) {
        return Err("分支名不能包含特殊字符：~, ^, :, ?, *, [, \\, 空格等".into());
    }

    if name.len() > 255 {
        return Err("分支名太长".into());
    }

    Ok(())
}

/// 从 HEAD 读取当前指向的 commit hash，不论是否为分支
pub fn read_head_commit_hash(repo_path: &Path) -> io::Result<String> {
    let head_path = repo_path.join("HEAD");
    let head_content = fs::read_to_string(&head_path)?.trim().to_string();

    if head_content.starts_with("ref: ") {
        let ref_path = repo_path.join(head_content.trim_start_matches("ref: ").trim());
        fs::read_to_string(&ref_path).map(|s| s.trim().to_string())
    } else {
        Ok(head_content)
    }
}
