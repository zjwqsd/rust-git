use std::fs::{self};
use std::io::{self};
use std::path::{Path};
use crate::core::{index::read_index, tree::create_tree};
use crate::core::reference::read_head_commit_hash;
use crate::utils::hash::sha1_hash;
use std::collections::HashSet;
use crate::core::config::IS_VERBOSE;

pub fn create_commit(message: &str, repo_path: &Path) -> io::Result<String> {
    let entries = read_index(&repo_path.join("index"))?;
    // println!("📦 准备生成 tree，当前 index 中的条目:");
    // for (hash, path) in &entries {
    //     println!("    {} {}", hash, path);
    // }

    let tree_hash = create_tree(&entries, repo_path)?;

    // 获取 HEAD 内容（可能是 symbolic ref，也可能是 commit hash）
    let head_path = repo_path.join("HEAD");
    let head_content = fs::read_to_string(&head_path)?.trim().to_string();
    // println!("📌 当前 HEAD 内容: {}", head_content);
    // 获取 parent commit（如果存在）
    let parent = match read_head_commit_hash(repo_path) {
        Ok(commit) if !commit.is_empty() => Some(commit),
        _ => None,
    };


    let author = "Your Name <you@example.com>";
    let content = format!(
        "tree {}\n{}author {}\ncommitter {}\n\n{}",
        tree_hash,
        if let Some(p) = &parent { format!("parent {}\n", p) } else { "".into() },
        author,
        author,
        message
    );

    let hash = sha1_hash(content.as_bytes());
    let (dir, file) = hash.split_at(2);
    let obj_dir = repo_path.join("objects").join(dir);
    fs::create_dir_all(&obj_dir)?;
    let path = obj_dir.join(file);
    fs::write(path, content)?;

    // 更新 HEAD 或分支引用
    if head_content.starts_with("ref: ") {
        let ref_path = repo_path.join(head_content.trim_start_matches("ref: ").trim());
        if *IS_VERBOSE {
            println!("🔗 更新分支 {} -> {}", ref_path.display(), hash);
        }
        fs::write(ref_path, format!("{}\n", hash))?;
    } else {
        // detached HEAD
        if *IS_VERBOSE {
            println!("🔗 更新 HEAD -> {}", hash);
        }
        fs::write(head_path, format!("{}\n", hash))?;
    }

    Ok(hash)
}



/// 从提交对象中读取 tree 哈希
pub fn read_commit_tree(commit_hash: &str, repo_path: &Path) -> io::Result<String> {
    let (dir, file) = commit_hash.split_at(2);
    let path = repo_path.join("objects").join(dir).join(file);
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if line.starts_with("tree ") {
            return Ok(line[5..].trim().to_string());
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "找不到 tree"))
}

pub fn create_merge_commit(
    repo_path: &Path,
    tree_hash: &str,
    parent1: &str,
    parent2: &str,
    message: &str,
) -> io::Result<String> {
    let author = "Your Name <you@example.com>";
    let content = format!(
        "tree {}\nparent {}\nparent {}\nauthor {}\ncommitter {}\n\n{}",
        tree_hash,
        parent1,
        parent2,
        author,
        author,
        message
    );

    let hash = sha1_hash(content.as_bytes());
    let (dir, file) = hash.split_at(2);
    let obj_dir = repo_path.join("objects").join(dir);
    fs::create_dir_all(&obj_dir)?;
    let path = obj_dir.join(file);
    fs::write(path, content)?;

    Ok(hash)
}

/// 向上追溯所有祖先
fn collect_ancestors(mut commit: String, repo: &Path) -> HashSet<String> {
    let mut ancestors = HashSet::new();

    while commit.len() >= 2 {
        ancestors.insert(commit.clone());

        let (dir, file) = commit.split_at(2);
        let path = repo.join("objects").join(dir).join(file);
        if !path.exists() {
            break;
        }

        let content = fs::read_to_string(&path).unwrap_or_default();
        if let Some(parent_line) = content.lines().find(|line| line.starts_with("parent ")) {
            commit = parent_line[7..].to_string(); // skip "parent "
        } else {
            break; // no parent = root
        }
    }

    ancestors
}

/// 查找共同祖先（第一个相交的）
pub fn find_common_ancestor(
    current: &str,
    target: &str,
    repo: &Path,
) -> Option<String> {
    let current_ancestors = collect_ancestors(current.to_string(), repo);
    let mut t = target.to_string();

    while t.len() >= 2 {
        if current_ancestors.contains(&t) {
            return Some(t);
        }

        let (dir, file) = t.split_at(2);
        let path = repo.join("objects").join(dir).join(file);
        if !path.exists() {
            break;
        }

        let content = fs::read_to_string(&path).unwrap_or_default();
        if let Some(parent_line) = content.lines().find(|line| line.starts_with("parent ")) {
            t = parent_line[7..].to_string();
        } else {
            break;
        }
    }

    None
}

