// tests/init.rs

use assert_cmd::Command;
use tempfile::tempdir;
// use std::fs;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

#[test]
fn test_init_default_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init")
        .current_dir(repo)
        .assert()
        .success();

    // 检查 .mygit 目录和 HEAD 文件
    assert!(repo.join(".mygit").exists());
    assert!(repo.join(".mygit/HEAD").exists());
}

// #[test]
// fn test_init_with_custom_branch() {
//     let tmp = tempdir().unwrap();
//     let repo = tmp.path();
//
//     bin().args(["init", "--initial-branch", "main"])
//         .current_dir(repo)
//         .assert()
//         .success();
//
//     let head = fs::read_to_string(repo.join(".mygit/HEAD")).unwrap();
//     assert!(head.contains("refs/heads/main"));
// }
