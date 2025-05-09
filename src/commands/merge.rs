use std::fs;
use std::path::Path;

use crate::core::commit::{read_commit_tree, is_ancestor_commit,create_merge_commit};
use crate::core::reference::{get_head_ref, get_current_branch_name};
use crate::core::tree::restore_tree;
use crate::core::tree::{read_tree_entries, load_blob,merge_tree_simple,write_tree_from_map};

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

    // 获取两个分支的提交哈希
    let target_ref = repo_path.join("refs/heads").join(target_branch);
    if !target_ref.exists() {
        eprintln!("目标分支 '{}' 不存在", target_branch);
        return;
    }

    let current_commit = fs::read_to_string(&head_ref_path).unwrap().trim().to_string();
    let target_commit = fs::read_to_string(&target_ref).unwrap().trim().to_string();
    if target_commit.len() < 2 {
        eprintln!("目标分支尚未提交，无法合并");
        return;
    }

    if current_commit.len() < 2 {
        eprintln!("当前分支尚未提交，无法执行合并");
        return;
    }
    // 检查是否可以 Fast-forward
    // if !is_ancestor_commit(&current_commit, &target_commit, repo_path) {
    //     eprintln!("无法 fast-forward，当前实现仅支持 fast-forward 合并");
    //     return;
    // }
    // if !is_ancestor_commit(&current_commit, &target_commit, repo_path) {
    if true{
        // 执行冲突检测（非 fast-forward）

        let current_tree_hash = read_commit_tree(&current_commit, repo_path).unwrap();
        let target_tree_hash = read_commit_tree(&target_commit, repo_path).unwrap();

        let current_tree = read_tree_entries(&current_tree_hash, repo_path).unwrap(); // HashMap<String, String>
        let target_tree = read_tree_entries(&target_tree_hash, repo_path).unwrap();

        let mut conflict_found = false;

        for (file, target_hash) in &target_tree {
            if let Some(current_hash) = current_tree.get(file) {
                if current_hash != target_hash {
                    // 内容不同，检查冲突行
                    let current_lines = load_blob(current_hash, repo_path).unwrap();
                    let target_lines = load_blob(target_hash, repo_path).unwrap();

                    // let mut conflicts = Vec::new();
                    let max_lines = current_lines.len().max(target_lines.len());

                    let mut i = 0;
                    while i < max_lines {
                        let line1 = current_lines.get(i);
                        let line2 = target_lines.get(i);

                        if line1 != line2 {
                            // 找冲突段区间 [start, end]
                            let start = i + 1;
                            let mut end = start;

                            while i < max_lines {
                                let l1 = current_lines.get(i);
                                let l2 = target_lines.get(i);
                                if l1 == l2 {
                                    break;
                                }
                                end = i + 1;
                                i += 1;
                            }

                            let file_display = Path::new(file)
                                .file_name()
                                .unwrap_or_else(|| file.as_ref())
                                .to_string_lossy();

                            if start == end {
                                println!("Merge conflict in {}: {}", file_display, start);
                            } else {
                                println!("Merge conflict in {}: [{}-{}]", file_display, start, end);
                            }


                            conflict_found = true;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        }

        if !conflict_found {
            println!("存在分叉但无冲突");
            let merged_tree = merge_tree_simple(&current_tree, &target_tree);
            let new_tree_hash = write_tree_from_map(&merged_tree, repo_path).unwrap();
            let merge_commit_hash = create_merge_commit(
                repo_path,
                &new_tree_hash,
                &current_commit,
                &target_commit,
                &format!("Merge branch '{}' into '{}'", target_branch, current_branch),
            ).unwrap();

            // 更新 HEAD
            fs::write(&head_ref_path, format!("{}\n", merge_commit_hash)).unwrap();

            // 恢复工作区
            restore_tree(&new_tree_hash, repo_path).unwrap();

            println!("已合并分支 '{}'（创建合并提交）", target_branch);

        }

        return;
    }

    // 执行 Fast-forward
    fs::write(&head_ref_path, format!("{}\n", target_commit)).unwrap();

    // 更新工作区
    let tree_hash = read_commit_tree(&target_commit, repo_path).unwrap();
    restore_tree(&tree_hash, repo_path).unwrap();

    println!("已 fast-forward 合并分支 '{}'", target_branch);
}
