use std::fs;
use std::path::Path;
use crate::core::config::IS_VERBOSE;
use crate::core::index::{remove_from_index, remove_directory_entries_from_index};

pub fn git_rm(file: &str, recursive: bool) {
    let path = Path::new(file);

    // 删除工作区文件或目录
    if path.exists() {
        let result = if path.is_file() {
            fs::remove_file(path)
        } else if path.is_dir() && recursive {
            fs::remove_dir_all(path)
        } else if path.is_dir() && !recursive {
            if *IS_VERBOSE {
                eprintln!("{} 是一个目录，请使用 -r 参数递归删除", file);
            }
            return;
        } else {
            if *IS_VERBOSE {
                eprintln!("无法识别的路径类型: {}", file);
            }
            return;
        };

        if let Err(e) = result {
            if *IS_VERBOSE {
                eprintln!("删除工作区文件/目录失败: {}", e);
            }
            return;
        } else {
            if *IS_VERBOSE {
                println!("已删除工作区中的: {}", file);
            }
        }
    }

    // 从 index 中移除（无论是文件还是目录内的所有文件）
    if recursive {
        remove_directory_entries_from_index(path);
    } else {
        match remove_from_index(path) {
            Ok(Some(_)) => {if *IS_VERBOSE{ println!("从暂存区移除: {}", file)}},
            Ok(None) => {if *IS_VERBOSE{println!("文件 {} 不在暂存区中，但将从提交中排除（若存在）", file)}},
            Err(e) => {if *IS_VERBOSE{eprintln!("更新 index 失败: {}", e)}},
        }
    }
}
