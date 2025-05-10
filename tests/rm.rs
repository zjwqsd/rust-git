// tests/rm.rs

use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

#[test]
fn test_rm_directory() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    fs::create_dir(repo.join("dir")).unwrap();
    fs::write(repo.join("dir/file.txt"), "test").unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "dir"]).current_dir(repo).assert().success();
    bin().args(["rm", "dir"]).current_dir(repo).assert().success();
}

// #[test]
// fn test_rm_cached_tracked_file() {
//     let tmp = tempdir().unwrap();
//     let repo = tmp.path();
//     let file = repo.join("file.txt");
//     fs::write(&file, "content").unwrap();
//
//     bin().arg("init").current_dir(repo).assert().success();
//     bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
//     bin().args(["rm", "--cached", "file.txt"]).current_dir(repo).assert().success();
// }

#[test]
fn test_rm_file_absolute_path() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let file = repo.join("file.txt");
    fs::write(&file, "content").unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();

    let abs_path = fs::canonicalize(&file).unwrap();
    bin().args(["rm", abs_path.to_str().unwrap()]).current_dir(repo).assert().success();
}

#[test]
fn test_rm_twice_same_file() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let file = repo.join("file.txt");
    fs::write(&file, "content").unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "file.txt"]).current_dir(repo).assert().success();
}