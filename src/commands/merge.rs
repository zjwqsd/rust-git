use std::fs;
use std::path::Path;

use crate::core::commit::{read_commit_tree, create_merge_commit};
use crate::core::reference::{get_head_ref, get_current_branch_name};
use crate::core::tree::{
    read_tree_entries, load_blob, merge_tree_simple, write_tree_from_map, restore_tree,
};

pub fn git_merge(target_branch: &str) {
    let repo_path = Path::new(".mygit");

    // 获取当前分支
    let head_ref_path = match get_head_ref(repo_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("无法获取 HEAD 引用: {}", e);
            return;
        }
    };

    let current_branch = match get_current_branch_name(repo_path) {
        Some(name) => name,
        None => {
            eprintln!("当前 HEAD 非分支引用");
            return;
        }
    };

    if current_branch == target_branch {
        println!("已经在分支 '{}'", target_branch);
        return;
    }

    // 获取提交哈希
    let target_ref = repo_path.join("refs/heads").join(target_branch);
    if !target_ref.exists() {
        eprintln!("目标分支 '{}' 不存在", target_branch);
        return;
    }

    let current_commit = fs::read_to_string(&head_ref_path).unwrap().trim().to_string();
    let target_commit = fs::read_to_string(&target_ref).unwrap().trim().to_string();

    if current_commit.len() < 2 || target_commit.len() < 2 {
        eprintln!("当前或目标分支尚未提交，无法合并");
        return;
    }

    // 读取两棵 tree
    let current_tree_hash = read_commit_tree(&current_commit, repo_path).unwrap();
    let target_tree_hash = read_commit_tree(&target_commit, repo_path).unwrap();

    let current_tree = read_tree_entries(&current_tree_hash, repo_path).unwrap();
    let target_tree = read_tree_entries(&target_tree_hash, repo_path).unwrap();

    // 简单冲突检测
    let mut conflict_found = false;
    for (file, target_hash) in &target_tree {
        if let Some(current_hash) = current_tree.get(file) {
            if current_hash != target_hash {
                let current_lines = load_blob(current_hash, repo_path).unwrap_or_default();
                let target_lines = load_blob(target_hash, repo_path).unwrap_or_default();

                let max_lines = current_lines.len().max(target_lines.len());
                let mut i = 0;
                while i < max_lines {
                    let line1 = current_lines.get(i);
                    let line2 = target_lines.get(i);
                    if line1 != line2 {
                        let start = i + 1;
                        let mut end = start;
                        while i < max_lines {
                            if current_lines.get(i) == target_lines.get(i) {
                                break;
                            }
                            end = i + 1;
                            i += 1;
                        }
                        let display_name = Path::new(file)
                            .file_name()
                            .unwrap_or_else(|| file.as_ref())
                            .to_string_lossy();
                        if start == end {
                            println!("Merge conflict in {}: {}", display_name, start);
                        } else {
                            println!("Merge conflict in {}: [{}-{}]", display_name, start, end);
                        }
                        conflict_found = true;
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }

    if conflict_found {
        println!("❗ 冲突发生，请手动解决");
        return;
    }

    println!("存在分叉但无冲突");

    // 合并 tree 并写入
    let merged_tree = merge_tree_simple(&current_tree, &target_tree);
    let new_tree_hash = write_tree_from_map(&merged_tree, repo_path).unwrap();

    // 创建合并提交
    let merge_commit_hash = create_merge_commit(
        repo_path,
        &new_tree_hash,
        &current_commit,
        &target_commit,
        &format!("Merge branch '{}' into '{}'", target_branch, current_branch),
    )
        .unwrap();

    // 更新 HEAD
    fs::write(&head_ref_path, format!("{}\n", merge_commit_hash)).unwrap();

    // 恢复文件
    restore_tree(&new_tree_hash, repo_path).unwrap();

    println!("已合并分支 '{}'（创建合并提交）", target_branch);
}
