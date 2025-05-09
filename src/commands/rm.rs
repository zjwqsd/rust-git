use std::fs;
use std::path::Path;

use crate::core::index::remove_from_index;

pub fn git_rm(file: &str) {
    let path = Path::new(file);

    // 删除工作区文件
    if path.exists() {
        if let Err(e) = fs::remove_file(path) {
            eprintln!("删除工作区文件失败: {}", e);
            return;
        } else {
            println!("删除工作区文件: {}", file);
        }
    }

    // 从 index 中移除
    match remove_from_index(path) {
        Ok(Some(_)) => {
            println!("从暂存区移除: {}", file);
        }
        Ok(None) => {
            println!("文件 {} 不在暂存区中，但将从提交中排除（若存在）", file);
        }
        Err(e) => eprintln!("更新 index 失败: {}", e),
    }
}
