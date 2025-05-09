use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::index::read_index;
use crate::utils::hash::sha1_hash;
use crate::utils::fs::list_files;

pub fn git_status() {
    let repo_path = Path::new(".mygit");
    let index_path = repo_path.join("index");

    let index_entries = read_index(&index_path).unwrap_or_default();
    let index_map: std::collections::HashMap<_, _> = index_entries
        .iter()
        .map(|(hash, path)| (path.clone(), hash.clone()))
        .collect();

    let mut seen: HashSet<String> = HashSet::new();

    // 修改检测
    for (path, old_hash) in &index_map {
        let path_buf = PathBuf::from(path);
        if let Ok(content) = fs::read(&path_buf) {
            let new_hash = sha1_hash(&content);
            if &new_hash != old_hash {
                println!("modified: {}", path);
            }
        } else {
            println!("deleted: {}", path);
        }
        seen.insert(path.clone());
    }

    // 未跟踪文件
    for file in list_files(Path::new(".")) {
        let rel = file.strip_prefix(".").unwrap_or(&file).to_string_lossy().to_string();
        if rel.starts_with(".mygit") {
            continue;
        }
        if !seen.contains(&rel) {
            println!("untracked: {}", rel);
        }
    }

    // 已暂存
    for path in index_map.keys() {
        println!("staged: {}", path);
    }
}
