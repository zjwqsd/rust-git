// tests/add.rs

use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

#[test]
fn test_add_single_file() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init")
        .current_dir(repo)
        .assert()
        .success();

    fs::write(repo.join("file.txt"), "hello").unwrap();
    bin().args(["add", "file.txt"])
        .current_dir(repo)
        .assert()
        .success();
}

#[test]
fn test_add_directory() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let dir = repo.join("subdir");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("a.txt"), "a").unwrap();
    fs::write(dir.join("b.txt"), "b").unwrap();

    bin().arg("init")
        .current_dir(repo)
        .assert()
        .success();

    bin().args(["add", "subdir"])
        .current_dir(repo)
        .assert()
        .success();
}

#[test]
fn test_add_file_and_directory() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    fs::create_dir(repo.join("sub")).unwrap();
    fs::write(repo.join("file.txt"), "hello").unwrap();
    fs::write(repo.join("sub/inside.txt"), "world").unwrap();

    bin().arg("init")
        .current_dir(repo)
        .assert()
        .success();

    bin().args(["add", "file.txt"])
        .current_dir(repo)
        .assert()
        .success();

    bin().args(["add", "sub"])
        .current_dir(repo)
        .assert()
        .success();
}
