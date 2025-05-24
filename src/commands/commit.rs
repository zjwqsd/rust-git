use crate::core::commit::create_commit;
// use std::path::Path;
use crate::core::config::{GIT_DIR};
pub fn git_commit(message: &str) {
    match create_commit(message, &*GIT_DIR) {
        // Ok(hash) => println!("已创建提交: {}", hash),
        Ok(hash) => println!("{}", hash),
        Err(e) => eprintln!("提交失败: {}", e),
    }
}
