use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crate::core::blob::write_blob;
/// 添加单个文件
fn add_file_to_index(file_path: &Path, index_file: &mut fs::File) -> io::Result<()> {
    let hash = write_blob(file_path)?;

    // ✅ 写入相对路径
    let cwd = std::env::current_dir().unwrap_or(PathBuf::from(""));
    let abs = fs::canonicalize(file_path)?;
    let rel_path = abs.strip_prefix(&cwd).unwrap_or(&abs);
    let clean_path = rel_path.to_string_lossy();

    writeln!(index_file, "{} {}", hash, clean_path)?;
    Ok(())
}

/// 新版 add_to_index 支持递归 add .
pub fn add_to_index(path: &Path) -> io::Result<()> {
    let index_path = Path::new(".mygit").join("index");
    let mut index_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(index_path)?;

    let current_exe = std::env::current_exe().ok();

    if path.is_file() {
        if let Some(ref exe) = current_exe {
            if path == exe {
                return Ok(()); // ✅ 忽略 rust-git 可执行文件
            }
        }
        add_file_to_index(path, &mut index_file)?;
    } else if path.is_dir() {
        visit_dir_recursively(path, &mut index_file, &current_exe)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "路径不存在"));
    }

    Ok(())
}

/// 遍历目录所有文件，递归实现
fn visit_dir_recursively(dir: &Path, index_file: &mut fs::File, current_exe: &Option<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        // ✅ 跳过 .mygit 目录
        if entry_path.is_dir() && entry_path.file_name().unwrap() == ".mygit" {
            continue;
        }

        // ✅ 跳过 rust-git 可执行文件
        if let Some(ref exe) = current_exe {
            if &entry_path == exe {
                continue;
            }
        }

        if entry_path.is_dir() {
            visit_dir_recursively(&entry_path, index_file, current_exe)?;
        } else if entry_path.is_file() {
            add_file_to_index(&entry_path, index_file)?;
        }
    }
    Ok(())
}

// pub fn read_index(index_path: &Path) -> io::Result<Vec<(String, String)>> {
//     let content = fs::read_to_string(index_path)?;
//     let mut result = Vec::new();
//     for line in content.lines() {
//         if let Some((hash, path)) = line.split_once(' ') {
//             result.push((hash.to_string(), path.to_string()));
//         }
//     }
//     Ok(result)
// }
pub fn read_index(index_path: &Path) -> io::Result<Vec<(String, String)>> {
    let content = fs::read_to_string(index_path)?;
    let mut entries = Vec::new();

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(""));

    for line in content.lines() {
        if let Some((hash, path)) = line.split_once(' ') {
            let clean_path = Path::new(path)
                .strip_prefix(&cwd)
                .unwrap_or(Path::new(path))
                .to_string_lossy()
                .to_string();

            entries.push((hash.to_string(), clean_path));
        }
    }
    for (hash, path) in &entries {
        println!("📥 index 读取: {} -> {}", hash, path);
    }
    Ok(entries)
}
/// 从 index 中删除某个文件条目，并返回其 hash（用于删除对象）
pub fn remove_from_index(path: &Path) -> io::Result<Option<String>> {
    let index_path = Path::new(".mygit/index");
    if !index_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(index_path)?;
    let mut new_lines = Vec::new();
    let mut removed_hash = None;

    // 👇 计算 path 的规范相对路径
    let cwd = std::env::current_dir()?;
    let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let rel_path = abs_path.strip_prefix(&cwd).unwrap_or(&abs_path);
    let rel_path_str = rel_path.to_string_lossy();

    for line in content.lines() {
        if let Some((hash, entry_path)) = line.split_once(' ') {
            if entry_path == rel_path_str {
                removed_hash = Some(hash.to_string());
                println!("✅ 从 index 中移除: {}", entry_path);
                continue;
            }
        }
        new_lines.push(line.to_string());
    }

    fs::write(index_path, new_lines.join("\n"))?;

    if removed_hash.is_none() {
        println!("⚠️ 无法匹配 index 路径: {}", rel_path_str);
    }else {
        println!("⚠️ 未从 index 中移除: {}", rel_path_str);
    }

    Ok(removed_hash)
}

// if removed.is_some() {
// println!("✅ 从 index 中移除: {}", rel_path);
// } else {
// println!("⚠️ 未从 index 中移除: {}", rel_path);
// }