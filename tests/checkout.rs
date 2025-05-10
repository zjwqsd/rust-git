// tests/checkout.rs

use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

#[test]
fn test_checkout_b_creates_and_switches_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "main").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "main commit"]).current_dir(repo).assert().success();

    bin().args(["checkout", "-b", "feature"]).current_dir(repo).assert().success();
    assert!(repo.join(".mygit/refs/heads/feature").exists());
}

#[test]
fn test_checkout_from_branch_to_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "a").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a"]).current_dir(repo).assert().success();

    bin().args(["branch", "dev"]).current_dir(repo).assert().success();
    bin().args(["checkout", "dev"]).current_dir(repo).assert().success();
}

#[test]
fn test_checkout_to_detached_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "a").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a"]).current_dir(repo).assert().success();

    let hash = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    bin().args(["checkout", &hash]).current_dir(repo).assert().success();
}

#[test]
fn test_checkout_from_detached_to_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "a").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a"]).current_dir(repo).assert().success();

    let hash = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    bin().args(["checkout", &hash]).current_dir(repo).assert().success();
    bin().args(["checkout", "main"]).current_dir(repo).assert().success();
}

#[test]
fn test_checkout_detached_to_another_detached() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "v1"]).current_dir(repo).assert().success();

    fs::write(repo.join("file.txt"), "v2").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "v2"]).current_dir(repo).assert().success();

    let commits = fs::read_to_string(repo.join(".mygit/refs/heads/main")).unwrap();
    let commit2 = commits.trim().to_string();
    let commit1 = fs::read_to_string(repo.join(".mygit/objects")
        .join(&commit2[0..2])
        .join(&commit2[2..]))
        .unwrap()
        .lines()
        .find(|l| l.starts_with("parent "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_string();

    bin().args(["checkout", &commit2]).current_dir(repo).assert().success();
    bin().args(["checkout", &commit1]).current_dir(repo).assert().success();
}

#[test]
fn test_checkout_branch_from_detached_head_should_inherit_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    // 初始化但不切换到 main，保持 detached 状态
    bin().arg("init").current_dir(repo).assert().success();

    // 提交一个文件
    fs::write(repo.join("foo.txt"), "hello").unwrap();
    bin().args(["add", "foo.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "initial"]).current_dir(repo).assert().success();

    // 此时 HEAD 是 detached，立即创建分支
    bin().args(["checkout", "-b", "test"]).current_dir(repo).assert().success();

    // 读取 test 分支引用
    let test_ref = repo.join(".mygit/refs/heads/test");
    let commit_hash = fs::read_to_string(test_ref).unwrap().trim().to_string();

    // 检查该 commit 是否存在 blob foo.txt
    let (dir, file) = commit_hash.split_at(2);
    let commit_path = repo.join(".mygit/objects").join(dir).join(file);
    let commit_content = fs::read_to_string(commit_path).unwrap();

    assert!(
        commit_content.contains("foo.txt"),
        "新创建的分支未继承 HEAD 提交，foo.txt 不在 commit 中"
    );
}