use std::fs::{self};
use std::io::{self};
use std::path::{Path};
use crate::core::{index::read_index, tree::create_tree,reference::get_head_ref};
use crate::utils::hash::sha1_hash;

pub fn create_commit(message: &str, repo_path: &Path) -> io::Result<String> {
    let entries = read_index(&repo_path.join("index"))?;
    let tree_hash = create_tree(&entries, repo_path)?;

    // let head_ref = repo_path.join("refs/heads/main");
    let head_ref_path = get_head_ref(repo_path)?;
    let parent = if head_ref_path.exists() {
        Some(fs::read_to_string(&head_ref_path)?.trim().to_string())
    } else {
        None
    };


    let author = "Your Name <you@example.com>";
    let content = format!(
        "tree {}\n{}{}\nauthor {}\ncommitter {}\n\n{}",
        tree_hash,
        if let Some(p) = &parent { format!("parent {}\n", p) } else { "".into() },
        "",
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

    // 更新 HEAD 指针
    // fs::write(head_ref, format!("{}\n", hash))?;
    fs::write(&head_ref_path, format!("{}\n", hash))?;
    // 清空 index
    fs::write(repo_path.join("index"), "")?;

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
/// 判断 base_commit 是否为 target_commit 的祖先（仅查一层 parent）
pub fn is_ancestor_commit(base: &str, target: &str, repo_path: &Path) -> bool {
    if base.len() < 2 || target.len() < 2 {
        eprintln!("提交哈希无效：base={}, target={}", base, target);
        return false;
    }

    let mut current = Some(target.to_string());

    while let Some(hash) = current {
        if hash == base {
            return true;
        }

        if hash.len() < 2 {
            break;
        }

        let (dir, file) = hash.split_at(2);
        let path = repo_path.join("objects").join(dir).join(file);
        let content = fs::read_to_string(path).unwrap_or_default();

        current = None;
        for line in content.lines() {
            if line.starts_with("parent ") {
                let parent_hash = line[7..].trim();
                current = Some(parent_hash.to_string());
                break;
            }
        }
    }

    false
}

