use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::core::blob::write_blob;

/// 将路径标准化为统一格式（相对路径 + / 分隔符）
pub fn normalize_path(path: &Path) -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let abs = cwd.join(path); // ✅ 直接拼接，不调用 fs::canonicalize
    let rel = abs.strip_prefix(&cwd).unwrap_or(&abs);
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

/// 添加单个文件
fn add_file_to_index(file_path: &Path, index_file: &mut fs::File) -> io::Result<()> {
    let hash = write_blob(file_path)?;
    let clean_path = normalize_path(file_path)?;

    println!("📌 add_file_to_index: {} -> {}", file_path.display(), clean_path);
    writeln!(index_file, "{} {}", hash, clean_path)?;
    Ok(())
}

/// 公开接口：添加路径（文件或目录）到 index
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
                return Ok(()); // 跳过可执行文件
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
        let path = entry.path();

        if path.is_dir() && path.file_name().map_or(false, |n| n == ".mygit") {
            continue;
        }

        if let Some(ref exe) = current_exe {
            if &path == exe {
                continue;
            }
        }

        if path.is_file() {
            add_file_to_index(&path, index_file)?;
        } else if path.is_dir() {
            visit_dir_recursively(&path, index_file, current_exe)?;
        }
    }
    Ok(())
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

    // let index_path = Path::new(".mygit/index");
    let index_path = Path::new(".mygit").join("index");

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
