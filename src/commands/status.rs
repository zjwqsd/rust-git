use std::{fs, io};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use crate::core::index::read_index;
use crate::utils::hash::sha1_hash;
use crate::core::tree::read_tree_entries;

/// 读取 HEAD 所在的 commit 的 tree（路径 -> blob hash 映射）
fn read_head_tree_map(repo_path: &Path) -> io::Result<HashMap<String, String>> {
    let head_path = repo_path.join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;
    let head_ref = head_content
        .trim()
        .strip_prefix("ref: ")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "HEAD 格式错误"))?;

    let ref_path = repo_path.join(head_ref);
    if !ref_path.exists() {
        return Ok(HashMap::new()); // 首次 commit 前为空
    }

    let commit_hash = fs::read_to_string(&ref_path)?.trim().to_string();
    if commit_hash.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("无效的 commit hash: '{}'", commit_hash)));
    }
    let (dir, file) = commit_hash.split_at(2);

    let commit_path = repo_path.join("objects").join(dir).join(file);
    let commit_content = fs::read_to_string(commit_path)?;

    let tree_hash = commit_content
        .lines()
        .find(|line| line.starts_with("tree "))
        .map(|line| line[5..].trim())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "无法从 commit 中读取 tree"))?;

    read_tree_entries(tree_hash, repo_path)
}

pub fn git_status() {
    let repo_path = Path::new(".mygit");
    let index_path = repo_path.join("index");

    // 读取 index
    let index_entries = read_index(&index_path).unwrap_or_default();
    let index_map: HashMap<_, _> = index_entries
        .iter()
        .map(|(hash, path)| (path.clone(), hash.clone()))
        .collect();

    // 读取 HEAD 的 tree
    let head_map = read_head_tree_map(repo_path).unwrap_or_default();
    let mut seen: HashSet<String> = HashSet::new();

    // ✅ 1. 对比 HEAD 与 index：找出 staged 文件
    for (path, index_hash) in &index_map {
        match head_map.get(path) {
            Some(tree_hash) => {
                if tree_hash != index_hash {
                    println!("staged: {}", path); // 文件内容变更
                }
            }
            None => {
                println!("staged: {}", path); // 新增文件
            }
        }
        seen.insert(path.clone());
    }

    // ✅ 2. 对比 index 与工作区：找出 modified 或 deleted 文件
    for (path, index_hash) in &index_map {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            if let Ok(content) = fs::read(&path_buf) {
                let work_hash = sha1_hash(&content);
                if &work_hash != index_hash {
                    println!("modified: {}", path);
                }
            }
        } else {
            println!("deleted: {}", path);
        }
    }

    // ✅ 3. 工作目录中未在 index 中出现的 → untracked
    for file in crate::utils::fs::list_files(Path::new(".")) {
        let rel = file
            .strip_prefix(".")
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        if rel.starts_with(".mygit") {
            continue;
        }
        if !seen.contains(&rel) {
            println!("untracked: {}", rel);
        }
    }
}
