use std::fs;
use std::path::Path;
use crate::core::reference::{set_head,validate_branch_name};
use crate::core::commit::read_commit_tree;
use crate::core::tree::{restore_tree, clean_working_directory};

/// 判断是否是合法的 40 位 commit hash
fn is_commit_hash(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// 检查分支名是否合法。如果非法，返回 `Err(原因)`，合法则返回 `Ok(())`


/// 主函数：执行 checkout 逻辑
pub fn git_checkout(target: &str, create: bool) {
    let repo_path = Path::new(".mygit");

    // 🚫 拒绝直接使用 "ref: refs/..." 形式
    if target.starts_with("ref: ") {
        eprintln!("❌ 错误：不允许直接使用 'ref: ...' 作为参数，请使用分支名或 commit hash");
        return;
    }

    // 🆕 detached HEAD 模式
    if !create && is_commit_hash(target) {
        fs::write(repo_path.join("HEAD"), format!("{}\n", target)).unwrap();
        println!("🔗 已切换到 commit {}（detached HEAD）", target);

        if let Err(e) = clean_working_directory() {
            eprintln!("清理工作区失败: {}", e);
            return;
        }

        if let Ok(tree_hash) = read_commit_tree(target, repo_path) {
            if let Err(e) = restore_tree(&tree_hash, repo_path) {
                eprintln!("恢复工作区失败: {}", e);
            }
        } else {
            eprintln!("❌ 无法找到指定 commit 的 tree");
        }

        return;
    }

    // ✅ 校验分支名是否合法
    if let Err(reason) = validate_branch_name(target) {
        eprintln!("❌ 无效的分支名 '{}': {}", target, reason);
        return;
    }

    let ref_path = repo_path.join("refs/heads").join(target);

    if create {
        if ref_path.exists() {
            eprintln!("❌ 分支 '{}' 已存在", target);
            return;
        }

        // 获取当前 HEAD 指向的 commit hash
        let head_content = fs::read_to_string(repo_path.join("HEAD"))
            .unwrap_or_default()
            .trim()
            .to_string();

        let commit_hash = if head_content.starts_with("ref: ") {
            let head_ref_path = repo_path.join(head_content.trim_start_matches("ref: ").trim());
            fs::read_to_string(head_ref_path).unwrap_or_default().trim().to_string()
        } else {
            head_content
        };

        fs::write(&ref_path, format!("{}\n", commit_hash)).unwrap();
        println!("✅ 创建分支 '{}'", target);
    }

    // 分支切换
    if !ref_path.exists() {
        eprintln!("❌ 分支 '{}' 不存在", target);
        return;
    }

    if let Err(e) = set_head(&format!("refs/heads/{}", target), repo_path) {
        eprintln!("❌ 无法设置 HEAD: {}", e);
        return;
    }

    if let Err(e) = clean_working_directory() {
        eprintln!("清理工作区失败: {}", e);
        return;
    }

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
                println!("✅ 已切换到分支 '{}'", target);
            }
        }
        Err(e) => {
            eprintln!("❌ 无法读取提交 tree: {}", e);
        }
    }
}
