use std::fs;
use std::path::Path;
use crate::core::reference::{set_head, get_head_ref};
use crate::core::commit::read_commit_tree;
use crate::core::tree::{restore_tree, clean_working_directory};

pub fn git_checkout(branch: &str, create: bool) {
    let repo_path = Path::new(".mygit");
    let ref_path = repo_path.join("refs/heads").join(branch);

    if create {
        if ref_path.exists() {
            eprintln!("分支 '{}' 已存在", branch);
            return;
        }

        // 获取当前 HEAD 指向的提交（如果存在）
        let head_ref_path = match get_head_ref(repo_path) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("HEAD 无效，无法创建分支");
                return;
            }
        };

        let commit_hash = if head_ref_path.exists() {
            fs::read_to_string(&head_ref_path)
                .unwrap_or_default()
                .trim()
                .to_string()
        } else {
            String::new() // 无提交，允许空分支
        };

        fs::write(&ref_path, format!("{}\n", commit_hash)).unwrap();
        println!("创建分支 '{}'", branch);
    }

    if !ref_path.exists() {
        eprintln!("分支 '{}' 不存在", branch);
        return;
    }

    // 设置 HEAD
    if let Err(e) = set_head(&format!("refs/heads/{}", branch), repo_path) {
        eprintln!("无法设置 HEAD: {}", e);
        return;
    }

    // ✅ 清理当前工作区
    if let Err(e) = clean_working_directory() {
        eprintln!("清理工作区失败: {}", e);
        return;
    }

    // 恢复提交（如果该分支有提交）
    let commit_hash = fs::read_to_string(&ref_path)
        .unwrap_or_default()
        .trim()
        .to_string();

    if commit_hash.is_empty() {
        println!("提示：当前分支尚无提交，工作区为空（仅保留 .mygit）");
        return;
    }

    match read_commit_tree(&commit_hash, repo_path) {
        Ok(tree_hash) => {
            if let Err(e) = restore_tree(&tree_hash, repo_path) {
                eprintln!("恢复工作区失败: {}", e);
            } else {
                println!("已切换到分支 '{}'", branch);
            }
        }
        Err(e) => {
            eprintln!("无法读取提交 tree: {}", e);
        }
    }
}
