use crate::core::index::add_to_index;
use std::path::Path;
use crate::core::config::IS_VERBOSE;
pub fn git_add(file_path: &str) {
    let path = Path::new(file_path);
    let abs_path = match std::env::current_dir() {
        Ok(current) => current.join(path),
        Err(e) => {

            eprintln!("无法获取当前目录: {}", e);
            return;
        }
    };

    if let Err(e) = add_to_index(&abs_path) {
        eprintln!("添加文件失败: {}", e);
    } else {
        if *IS_VERBOSE {
            println!("已添加 {}", file_path);
        }
    }
}
