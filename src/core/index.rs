use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use crate::core::blob::write_blob;
use crate::core::config::{GIT_DIR};
/// 将路径标准化为统一格式（相对路径 + / 分隔符）
/// 将路径标准化为统一格式（相对路径 + / 分隔符）
pub fn normalize_path(path: &Path) -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let abs = cwd.join(path); // 绝对路径
    let rel = abs.strip_prefix(&cwd).unwrap_or(&abs); // 相对路径
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

/// 读取 index 内容为 map（path -> hash）
fn load_index(index_path: &Path) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();

    if let Ok(content) = fs::read_to_string(index_path) {
        for line in content.lines() {
            if let Some((hash, path)) = line.trim().split_once(' ') {
                map.insert(path.to_string(), hash.to_string());
            }
        }
    }

    map
}

/// 保存 index（path -> hash）为 index 文件
fn save_index(index_path: &Path, map: &BTreeMap<String, String>) -> io::Result<()> {
    let mut file = File::create(index_path)?;
    for (path, hash) in map {
        writeln!(file, "{} {}", hash, path)?;
    }
    Ok(())
}

/// 添加单个文件（更新 blob、替换 index 条目）
fn add_single_file(path: &Path, index: &mut BTreeMap<String, String>) -> io::Result<()> {
    let hash = write_blob(path)?;
    let rel_path = normalize_path(path)?;
    index.insert(rel_path.clone(), hash.clone());
    println!("✅ 添加到 index: {} -> {}", rel_path, hash);
    Ok(())
}

/// 遍历目录递归添加
fn add_dir_recursive(dir: &Path, index: &mut BTreeMap<String, String>, exe: &Option<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.file_name().map_or(false, |n| n == *GIT_DIR) {
            continue;
        }

        if let Some(ref exe_path) = exe {
            if &path == exe_path {
                continue;
            }
        }

        if path.is_file() {
            add_single_file(&path, index)?;
        } else if path.is_dir() {
            add_dir_recursive(&path, index, exe)?;
        }
    }
    Ok(())
}


// 公共接口：添加路径（文件或目录）到 index
pub fn add_to_index(path: &Path) -> io::Result<()> {
    let index_path = &*GIT_DIR.join("index");
    let mut index = load_index(&index_path);

    let exe = std::env::current_exe().ok();

    if path.is_file() {
        if let Some(ref exe_path) = exe {
            if path == exe_path {
                return Ok(()); // 跳过可执行文件
            }
        }
        add_single_file(path, &mut index)?;
    } else if path.is_dir() {
        add_dir_recursive(path, &mut index, &exe)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "路径不存在"));
    }

    save_index(&index_path, &index)
}


/// 读取 index 内容
pub fn read_index(index_path: &Path) -> io::Result<Vec<(String, String)>> {
    let content = fs::read_to_string(index_path)?;
    let mut entries = Vec::new();

    for line in content.lines() {
        if let Some((hash, path)) = line.split_once(' ') {
            entries.push((hash.to_string(), path.to_string()));
        }
    }

    for (hash, path) in &entries {
        println!("📥 index 读取: {} -> {}", hash, path);
    }

    Ok(entries)
}

/// 从 index 中删除文件记录
pub fn remove_from_index(path: &Path) -> io::Result<Option<String>> {
    println!("🔥 remove_from_index 正在运行");

    let index_path = &*GIT_DIR.join("index");

    if !index_path.exists() {
        println!("❗ 警告：index 文件不存在！路径是：{}", index_path.display());
        return Ok(None);
    }

    let content = fs::read_to_string(&index_path)?;
    println!("📄 index 原始内容:\n{}", content);

    let mut new_lines = Vec::new();
    let mut removed_hash = None;

    let target_path = normalize_path(path)?;
    println!("🎯 标准化目标路径: {}", target_path);

    for line in content.lines() {
        if let Some((hash, entry_path)) = line.split_once(' ') {
            if entry_path == target_path {
                println!("✅ 从 index 中移除: {}", entry_path);
                removed_hash = Some(hash.to_string());
                continue;
            } else {
                println!("❌ 匹配失败:");
                println!("   entry_path     = {:?}", entry_path);
                println!("   target_path    = {:?}", target_path);
                println!("   entry_path.bytes(): {:?}", entry_path.as_bytes());
                println!("   target_path.bytes(): {:?}", target_path.as_bytes());
            }
        }
        new_lines.push(line.to_string());
    }
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }
    println!("📄 最终写入 index 内容:\n{}", new_lines.join("\n"));

    fs::write(&index_path, new_lines.join("\n"))?;

    if removed_hash.is_none() {
        println!("⚠️ 未能匹配并移除 index 条目: {}", target_path);
    }

    Ok(removed_hash)
}

pub fn remove_directory_entries_from_index(dir_path: &Path) {
    let index_path = &*GIT_DIR.join("index");

    if !index_path.exists() {
        println!("⚠️ index 文件不存在");
        return;
    }

    let content = fs::read_to_string(&index_path).unwrap_or_default();
    let target_dir = normalize_path(dir_path).unwrap_or_default();

    let mut new_lines = Vec::new();
    for line in content.lines() {
        if let Some((_, entry_path)) = line.split_once(' ') {
            if !entry_path.starts_with(&target_dir) {
                new_lines.push(line.to_string());
            } else {
                println!("🗑️ 从 index 移除目录项: {}", entry_path);
            }
        }
    }

    fs::write(&index_path, new_lines.join("\n")).unwrap();
}