// tests/commit.rs

use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

#[test]
fn test_commit_normal() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "hello").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "add a.txt"]).current_dir(repo).assert().success();

    assert!(repo.join(".mygit/HEAD").exists());
}

#[test]
fn test_commit_with_chinese_message() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "你好").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "提交中文"]).current_dir(repo).assert().success();
}

#[test]
fn test_commit_empty_message() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "data").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();

    let result = bin().args(["commit", "-m", ""]).current_dir(repo).output().unwrap();
    assert!(result.status.success());
}

#[test]
fn test_rm_then_add_same_file_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "1").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "a.txt"]).current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "2").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "rm then add same name commit"]).current_dir(repo).assert().success();
}

#[test]
fn test_rm_then_add_then_add_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "old").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "a.txt"]).current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "new").unwrap();
    bin().args(["add", "."]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "rm then add . commit"]).current_dir(repo).assert().success();
}

#[test]
fn test_add_rm_then_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "something").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "add then rm commit"]).current_dir(repo).assert().success();
}

#[test]
fn test_add_rm_add_dot_then_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "one").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "a.txt"]).current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "two").unwrap();
    bin().args(["add", "."]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "add rm add again commit"]).current_dir(repo).assert().success();
}
