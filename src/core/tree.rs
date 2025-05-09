use std::fs::{self};
use std::io::{self};
use std::path::{Path};
use crate::utils::hash::sha1_hash;
use std::collections::HashMap;

pub fn clean_working_directory() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == "rust-git" || name == ".mygit" {
                continue; // 排除 rust-git 执行文件和 .mygit 目录
            }
        }
        if path.is_file() {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}
pub fn create_tree(entries: &[(String, String)], repo_path: &Path) -> io::Result<String> {
    let mut content = String::new();
    for (hash, path) in entries {
        content.push_str(&format!("blob {} {}\n", hash, path));
    }

    let tree_hash = sha1_hash(content.as_bytes());
    let (dir, file) = tree_hash.split_at(2);
    let obj_dir = repo_path.join("objects").join(dir);
    fs::create_dir_all(&obj_dir)?;
    let tree_path = obj_dir.join(file);
    fs::write(tree_path, content)?;

    Ok(tree_hash)
}

/// 还原 tree 中记录的文件
pub fn restore_tree(tree_hash: &str, repo_path: &Path) -> io::Result<()> {
    clean_working_directory()?;
    let (dir, file) = tree_hash.split_at(2);
    let tree_path = repo_path.join("objects").join(dir).join(file);
    let content = fs::read_to_string(&tree_path)?;

    for line in content.lines() {
        if let Some((_, rest)) = line.split_once("blob ") {
            if let Some((hash, filename)) = rest.split_once(' ') {
                let (obj_dir, obj_file) = hash.split_at(2);
                let blob_path = repo_path.join("objects").join(obj_dir).join(obj_file);

                let blob_content = fs::read(&blob_path)?;

                // 💡 强制覆盖文件（即使文件存在）
                fs::write(filename, blob_content)?;
                println!("✔ 恢复文件 {} -> {}", filename, hash);
            }
        }
    }

    Ok(())
}

/// 返回 tree 中所有文件及其 blob 哈希
pub fn read_tree_entries(tree_hash: &str, repo_path: &Path) -> std::io::Result<HashMap<String, String>> {
    let (dir, file) = tree_hash.split_at(2);
    let tree_path = repo_path.join("objects").join(dir).join(file);
    let content = fs::read_to_string(tree_path)?;

    let mut map = HashMap::new();
    for line in content.lines() {
        if let Some((_, rest)) = line.split_once("blob ") {
            if let Some((hash, path)) = rest.split_once(' ') {
                map.insert(path.to_string(), hash.to_string());
            }
        }
    }
    Ok(map)
}

/// 读取 blob 对象为 Vec<String>（按行）
pub fn load_blob(hash: &str, repo_path: &Path) -> std::io::Result<Vec<String>> {
    let (dir, file) = hash.split_at(2);
    let blob_path = repo_path.join("objects").join(dir).join(file);
    let content = fs::read_to_string(blob_path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// 合并两个 tree，保留所有不冲突文件
/// - 相同文件、相同 hash：保留
/// - 相同文件、不同 hash：跳过（冲突）
/// - 不同文件名：合并
pub fn merge_tree_simple(
    current: &HashMap<String, String>,
    target: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = current.clone();

    for (k, v) in target {
        match current.get(k) {
            Some(existing) => {
                if existing == v {
                    merged.insert(k.clone(), v.clone()); // 相同内容可以合并
                }
                // 否则冲突，跳过处理
            }
            None => {
                merged.insert(k.clone(), v.clone()); // 新文件添加
            }
        }
    }

    merged
}

/// 将 tree 的 HashMap 写入对象存储，返回 tree 哈希
pub fn write_tree_from_map(
    entries: &HashMap<String, String>,
    repo_path: &Path,
) -> std::io::Result<String> {
    use crate::utils::hash::sha1_hash;

    let mut content = String::new();
    for (filename, blob_hash) in entries {
        content.push_str(&format!("blob {} {}\n", blob_hash, filename));
    }

    let tree_hash = sha1_hash(content.as_bytes());
    let (dir, file) = tree_hash.split_at(2);
    let obj_dir = repo_path.join("objects").join(dir);
    fs::create_dir_all(&obj_dir)?;
    let tree_path = obj_dir.join(file);
    fs::write(tree_path, content)?;

    Ok(tree_hash)
}
