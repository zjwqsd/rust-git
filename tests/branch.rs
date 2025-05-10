// tests/branch.rs

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;
use std::fs;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

#[test]
fn test_branch_list_normal() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "data").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "init"]).current_dir(repo).assert().success();
    bin().args(["branch", "dev"]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("main")).stdout(contains("dev"));
}

#[test]
fn test_branch_list_in_detach() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "first commit"]).current_dir(repo).assert().success();
    let commit_hash = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    bin().args(["checkout", &commit_hash]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("main"));
}

#[test]
fn test_branch_create_in_detach() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "first commit"]).current_dir(repo).assert().success();
    let commit_hash = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    bin().args(["checkout", &commit_hash]).current_dir(repo).assert().success();
    bin().args(["branch", "new-branch"]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("new-branch"));
}

#[test]
fn test_branch_after_add() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "1").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["branch", "dev"]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("dev"));
}

#[test]
fn test_branch_after_rm() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("b.txt"), "b").unwrap();
    bin().args(["add", "b.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "b.txt"]).current_dir(repo).assert().success();
    bin().args(["branch", "x"]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("x"));
}

#[test]
fn test_branch_after_add_then_remove_file_manually() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let file = repo.join("foo.txt");

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(&file, "data").unwrap();
    bin().args(["add", "foo.txt"]).current_dir(repo).assert().success();
    std::fs::remove_file(&file).unwrap();
    bin().args(["branch", "lostfile"]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("lostfile"));
}

#[test]
fn test_branch_after_rm_add_same_file() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let file = repo.join("x.txt");

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(&file, "aaa").unwrap();
    bin().args(["add", "x.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "x.txt"]).current_dir(repo).assert().success();
    fs::write(&file, "aaa").unwrap();
    bin().args(["add", "x.txt"]).current_dir(repo).assert().success();
    bin().args(["branch", "resurrect"]).current_dir(repo).assert().success();
    bin().args(["branch"]).current_dir(repo).assert().stdout(contains("resurrect"));
}
