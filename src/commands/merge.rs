use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use crate::core::commit::{read_commit_tree, create_merge_commit,find_common_ancestor};
use crate::core::reference::{get_head_ref, get_current_branch_name};
use crate::core::tree::{
    read_tree_entries, load_blob, merge_tree_simple, write_tree_from_map, restore_tree,
};
use crate::core::config::{GIT_DIR};
pub fn git_merge(target_branch: &str) {
    let repo_path = &*GIT_DIR;

    // 获取 HEAD 和当前分支
    let head_ref_path = match get_head_ref(repo_path) {
        Ok(p) => p,
        Err(e) => return eprintln!("无法获取 HEAD: {}", e),
    };

    let current_branch = match get_current_branch_name(repo_path) {
        Some(b) => b,
        None => return eprintln!("当前 HEAD 不是分支"),
    };

    // if current_branch == target_branch {
    //     println!("已经在分支 '{}'", target_branch);
    //     return;
    // }

    let target_ref = repo_path.join("refs/heads").join(target_branch);
    if !target_ref.exists() {
        return eprintln!("目标分支 '{}' 不存在", target_branch);
    }

    let current_commit = fs::read_to_string(&head_ref_path).unwrap().trim().to_string();
    let target_commit = fs::read_to_string(&target_ref).unwrap().trim().to_string();

    if current_commit == target_commit {
        println!("Already up to Date");
        return;
    }
    if current_commit.len() < 2 || target_commit.len() < 2 {
        return eprintln!("当前或目标分支为空，无法合并");
    }

    // 读取三方 tree
    let current_tree_hash = read_commit_tree(&current_commit, repo_path).unwrap();
    let target_tree_hash = read_commit_tree(&target_commit, repo_path).unwrap();
    let current_tree = read_tree_entries(&current_tree_hash, repo_path).unwrap();
    let target_tree = read_tree_entries(&target_tree_hash, repo_path).unwrap();

    let base_tree = if let Some(base) =
        find_common_ancestor(&current_commit, &target_commit, repo_path)
    {
        let base_hash = read_commit_tree(&base, repo_path).unwrap();
        read_tree_entries(&base_hash, repo_path).unwrap_or_default()
    } else {
        HashMap::new() // 无共同祖先，视为初次提交
    };

    // 冲突检测（base-aware）
    let all_files: HashSet<_> = current_tree
        .keys()
        .chain(target_tree.keys())
        .collect();

    let mut conflict_found = false;

    for file in all_files {
        let base = base_tree.get(file);
        let cur = current_tree.get(file);
        let tgt = target_tree.get(file);

        // if base == cur || base == tgt {
        //     continue; // 一边没改，自动合并
        // }

        if cur != tgt && base != cur && base != tgt {
            let cur_lines = cur
                .and_then(|h| load_blob(h, repo_path).ok())
                .unwrap_or_default();
            let tgt_lines = tgt
                .and_then(|h| load_blob(h, repo_path).ok())
                .unwrap_or_default();

            let max_lines = cur_lines.len().max(tgt_lines.len());
            let mut i = 0;
            while i < max_lines {
                let l1 = cur_lines.get(i);
                let l2 = tgt_lines.get(i);
                if l1 != l2 {
                    let start = i + 1;
                    let mut end = start;
                    i += 1;
                    while i < max_lines && cur_lines.get(i) != tgt_lines.get(i) {
                        end = i + 1;
                        i += 1;
                    }
                    let name = Path::new(file)
                        .file_name()
                        .unwrap_or_else(|| file.as_ref())
                        .to_string_lossy();
                    if start == end {
                        println!("Merge conflict in {}: {}", name, start);
                    } else {
                        println!("Merge conflict in {}: [{}-{}]", name, start, end);
                    }
                    conflict_found = true;
                } else {
                    i += 1;
                }
            }
        }
    }

    if conflict_found {
        println!("❗ 冲突发生，请手动解决");
        return;
    }

    println!("存在分叉但无冲突");

    // 合并 tree（默认以目标为主）
    // let merged_tree = merge_tree_simple(&current_tree, &target_tree);
    let merged_tree = merge_tree_simple(&base_tree, &current_tree, &target_tree);

    let new_tree_hash = write_tree_from_map(&merged_tree, repo_path).unwrap();

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
    restore_tree(&new_tree_hash, repo_path).unwrap();
    println!("已合并分支 '{}'（创建合并提交）", target_branch);
}
