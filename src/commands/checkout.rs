use std::fs;
use std::path::Path;
use crate::core::reference::{set_head};
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

        // ✅ 正确读取当前 HEAD 的 commit（无论是否为 symbolic ref）
        let head_path = repo_path.join("HEAD");
        let head_content = fs::read_to_string(&head_path).unwrap_or_default().trim().to_string();
        println!("🧭 当前 HEAD 内容: {}", head_content);

        let commit_hash = if head_content.starts_with("ref: ") {
            // symbolic ref
            let head_ref_path = repo_path.join(head_content.trim_start_matches("ref: ").trim());
            fs::read_to_string(head_ref_path).unwrap_or_default().trim().to_string()
        } else {
            // detached HEAD
            println!("🧷 HEAD 为 detached，commit hash: {}", head_content);
            head_content
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

    // 清理工作区
    if let Err(e) = clean_working_directory() {
        eprintln!("清理工作区失败: {}", e);
        return;
    }

    // 读取新分支的提交并恢复
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
