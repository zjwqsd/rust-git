use std::fs;
// use std::io::{self, Write};
use std::io::{self};
// use std::path::Path;
use crate::core::reference::validate_branch_name;
use crate::core::config::{GIT_DIR, DEFAULT_BRANCH};
use crate::core::reference::get_current_branch_name;
pub fn git_branch(branch_name: Option<&str>) -> io::Result<()> {
    let repo_path = &*GIT_DIR; // 使用配置中的仓库路径
    let heads_dir = repo_path.join("refs/heads");

    if let Some(name) = branch_name {
        if let Err(reason) = validate_branch_name(name) {
            eprintln!("❌ 无效的分支名 '{}': {}", name, reason);
            std::process::exit(1); // 保证命令失败，测试可捕获
        }

        // 创建分支：从当前默认分支复制指针
        let head_ref = repo_path.join("refs/heads").join(&*DEFAULT_BRANCH);
        let current_commit = fs::read_to_string(&head_ref)?;
        let new_branch = heads_dir.join(name);
        fs::write(new_branch, current_commit)?;
        println!("✅ 已创建分支 '{}'", name);
    } else {
        // 列出所有分支
        let entries = fs::read_dir(&heads_dir)?;
        for entry in entries {
            let path = entry?.path();
            if let Some(name) = path.file_name() {
                println!("{}", name.to_string_lossy());
            }
        }
    }

    Ok(())
}

/// 删除分支
pub fn git_branch_delete(branch_name: &str) {
    let repo_path = &*GIT_DIR;
    let heads_dir = repo_path.join("refs/heads");
    let branch_path = heads_dir.join(branch_name);

    // 检查是否为当前分支
    let current = get_current_branch_name(repo_path);
    if let Some(current_name) = current {
        if current_name == branch_name {
            eprintln!("❌ 不能删除当前所在的分支 '{}'", branch_name);
            return;
        }
    }

    if branch_path.exists() {
        match fs::remove_file(&branch_path) {
            Ok(_) => println!("✅ 已删除分支 '{}'", branch_name),
            Err(e) => eprintln!("删除失败: {}", e),
        }
    } else {
        eprintln!("❌ 分支 '{}' 不存在", branch_name);
    }
}
