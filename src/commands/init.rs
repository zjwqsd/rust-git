use crate::core::repository::init_repository;
use std::path::Path;

pub fn git_init(target_path: &str) {
    let path = Path::new(target_path);
    match init_repository(path) {
        Ok(_) => println!("已在 {} 初始化空的Git仓库", path.display()),
        Err(e) => eprintln!("初始化失败: {}", e),
    }
}
