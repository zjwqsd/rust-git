use crate::core::commit::create_commit;
use std::path::Path;

pub fn git_commit(message: &str) {
    match create_commit(message, Path::new(".mygit")) {
        Ok(hash) => println!("已创建提交: {}", hash),
        Err(e) => eprintln!("提交失败: {}", e),
    }
}
