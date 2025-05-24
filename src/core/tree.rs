use std::fs::{self};
use std::io::{self};
use std::path::{Path};
use crate::utils::hash::sha1_hash;
use std::collections::HashMap;
use crate::core::config::{GIT_DIR, IS_VERBOSE};

/// 安全清理工作区，只保留 `Git 文件夹` 和执行文件本体
pub fn clean_working_directory() -> io::Result<()> {
    let exe = std::env::current_exe().ok();
    let mygit_path = fs::canonicalize(&*GIT_DIR).unwrap_or_else(|_| GIT_DIR.clone());

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        let canonical = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());

        if canonical.starts_with(&mygit_path) {
            if *IS_VERBOSE {
                println!("🔒 跳过 Git 文件夹内部文件或目录: {}", path.display());
            }
            continue;
        }

        if let Some(ref exe_path) = exe {
            if &canonical == exe_path {
                if *IS_VERBOSE {
                    println!("🔒 跳过当前可执行文件: {}", path.display());
                }
                continue;
            }
        }
        if *IS_VERBOSE {
            println!("检查路径: {}", path.display());
        }
        if path == *GIT_DIR {
            if *IS_VERBOSE {
                println!("🚨 警告: 竟然试图删除 Git 目录 {}!!!", GIT_DIR.display());
            }
        }
        if path.is_file() {
            if *IS_VERBOSE {
                println!("🧹 删除文件: {}", path.display());
            }
            fs::remove_file(&path)?;
        } else if path.is_dir() {
            if *IS_VERBOSE {
                println!("🧹 删除目录: {}", path.display());
            }
            fs::remove_dir_all(&path)?;
        }
    }
    Ok(())
}




pub fn create_tree(entries: &[(String, String)], repo_path: &Path) -> io::Result<String> {
    let mut content = String::new();

    for (hash, path) in entries {
        let file_path = Path::new(path);
        if file_path.exists() {
            content.push_str(&format!("blob {} {}\n", hash, path));
        } else {
            if *IS_VERBOSE {
                println!("⚠️  跳过不存在的文件 {}", path);
            }
        }
    }

    let tree_hash = sha1_hash(content.as_bytes());
    let (dir, file) = tree_hash.split_at(2);
    let obj_dir = repo_path.join("objects").join(dir);
    fs::create_dir_all(&obj_dir)?;
    let tree_path = obj_dir.join(file);
    // println!("🌲 最终写入 tree 对象内容：");
    // println!("{}", content);
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
                if *IS_VERBOSE {
                    println!("✔ 恢复文件 {} -> {}", filename, hash);
                }
            }
        }
    }

    Ok(())
}

/// 返回 tree 中所有文件及其 blob 哈希
pub fn read_tree_entries(tree_hash: &str, repo_path: &Path) -> io::Result<HashMap<String, String>> {
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


pub fn load_blob(hash: &str, repo_path: &Path) -> io::Result<Vec<String>> {
    let (dir, file) = hash.split_at(2);
    let blob_path = repo_path.join("objects").join(dir).join(file);
    let content = fs::read_to_string(blob_path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// 合并两个 tree，保留所有不冲突文件
/// - 相同文件、相同 hash：保留
/// - 相同文件、不同 hash：跳过（冲突）
/// - 不同文件名：合并
// pub fn merge_tree_simple(
//     current: &HashMap<String, String>,
//     target: &HashMap<String, String>,
// ) -> HashMap<String, String> {
//     let mut merged = HashMap::new();
//
//     for (path, hash) in target {
//         match current.get(path) {
//             Some(cur_hash) => {
//                 if cur_hash == hash {
//                     merged.insert(path.clone(), hash.clone()); // 内容一致，保留
//                 } else {
//                     merged.insert(path.clone(), hash.clone()); // 内容不同但无冲突，按目标分支覆盖
//                 }
//             }
//             None => {
//                 merged.insert(path.clone(), hash.clone()); // 新文件
//             }
//         }
//     }
//
//     // 🔥 特别注意：不要自动保留 current 中目标已删除的文件
//     // 即：如果 target 不包含某文件，则认为其被删除 → 不加入 merged
//
//     merged
// }
pub fn merge_tree_simple(
    base: &HashMap<String, String>,
    current: &HashMap<String, String>,
    target: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = HashMap::new();
    let all_files: std::collections::HashSet<_> =
        base.keys().chain(current.keys()).chain(target.keys()).collect();

    for file in all_files {
        let base_entry = base.get(file);
        let current_entry = current.get(file);
        let target_entry = target.get(file);

        match (base_entry, current_entry, target_entry) {
            // 当前没改，目标删除 => 删除
            (Some(base), Some(cur), None) if cur == base => {
                // 不加入 merged，相当于删除
            }

            // 当前改了，目标删除 => 保留当前（前面已判断无冲突）
            (Some(_base), Some(cur), None) => {
                merged.insert(file.clone(), cur.clone());
            }

            // 目标改了，当前删除 => 保留目标
            (Some(_base), None, Some(tgt)) => {
                merged.insert(file.clone(), tgt.clone());
            }

            // 文件只在 current 中（新增）
            (None, Some(cur), None) => {
                merged.insert(file.clone(), cur.clone());
            }

            // 文件只在 target 中（新增）
            (None, None, Some(tgt)) => {
                merged.insert(file.clone(), tgt.clone());
            }

            // 双方都有（内容一致或前面已判断无冲突）
            (_, Some(_), Some(tgt)) => {
                merged.insert(file.clone(), tgt.clone());
            }

            // 其他情况（如都删除），忽略
            _ => {}
        }
    }

    merged
}



/// 将 tree 的 HashMap 写入对象存储，返回 tree 哈希
pub fn write_tree_from_map(
    entries: &HashMap<String, String>,
    repo_path: &Path,
) -> io::Result<String> {
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
