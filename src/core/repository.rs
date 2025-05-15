use std::fs;
use std::io;
use std::path::Path;
// use crate::config::{GIT_DIR, DEFAULT_BRANCH};
use crate::core::config::{GIT_DIR,DEFAULT_BRANCH};
pub fn init_repository(path: &Path) -> io::Result<()> {
    // let git_dir = path.join(".mygit");
    let git_dir = path.join(&*GIT_DIR);
    if git_dir.exists() {
        return Ok(()); // 已初始化直接返回成功
    }

    fs::create_dir_all(&git_dir)?;

    // 创建 objects 目录
    let objects = git_dir.join("objects");
    fs::create_dir_all(objects.join("info"))?;
    fs::create_dir_all(objects.join("pack"))?;

    // 创建 refs 目录
    let refs = git_dir.join("refs");
    fs::create_dir_all(refs.join("heads"))?;
    fs::create_dir_all(refs.join("tags"))?;
    fs::create_dir_all(refs.join("remotes"))?;

    // 创建 HEAD 文件
    let head_path = git_dir.join("HEAD");
    // fs::write(&head_path, "ref: refs/heads/main\n")?;
    fs::write(&head_path, format!("ref: refs/heads/{}\n", *DEFAULT_BRANCH))?;
    // ✅ 创建空的 main 分支指针文件，防止后续找不到
    // let main_ref = git_dir.join("refs/heads/main");
    let main_ref = git_dir.join("refs/heads").join(&*DEFAULT_BRANCH);
    fs::write(&main_ref, "")?;

    Ok(())
}
