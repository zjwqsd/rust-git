// tests/rm.rs

use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;
// use std::path::Path;

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
    bin().args(["rm", "-r", "dir"]).current_dir(repo).assert().success();
}


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

#[test]
fn test_rm_empty_directory_recursive() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    let dir_path = repo.join("empty_dir");
    fs::create_dir(&dir_path).unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "empty_dir"]).current_dir(repo).assert().success();
    bin().args(["rm", "-r", "empty_dir"]).current_dir(repo).assert().success();
}


#[test]
fn test_rm_directory_twice() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    let dir = repo.join("dir");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("file.txt"), "test").unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "dir"]).current_dir(repo).assert().success();
    bin().args(["rm", "-r", "dir"]).current_dir(repo).assert().success();

    // 再次尝试删除（应成功或至少不 panic）
    bin().args(["rm", "-r", "dir"]).current_dir(repo).assert().success();
}

#[test]
fn test_rm_file_only_in_index() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let file = repo.join("file.txt");

    fs::write(&file, "hello").unwrap();
    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();

    // 手动删除工作区文件
    fs::remove_file(&file).unwrap();

    // 再调用 rm，应该从 index 移除
    bin().args(["rm", "file.txt"]).current_dir(repo).assert().success();
}

#[test]
fn test_rm_nested_file_in_directory() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let dir = repo.join("a/b/c");
    fs::create_dir_all(&dir).unwrap();

    let file = dir.join("deep.txt");
    fs::write(&file, "deep content").unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "a"]).current_dir(repo).assert().success();
    bin().args(["rm", "-r", "a"]).current_dir(repo).assert().success();
}

#[test]
fn test_rm_with_dot_dot_path() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    let subdir = repo.join("sub");
    fs::create_dir(&subdir).unwrap();

    let file = subdir.join("file.txt");
    fs::write(&file, "content").unwrap();

    bin().arg("init").current_dir(repo).assert().success();
    bin().args(["add", "sub/file.txt"]).current_dir(repo).assert().success();
    bin().args(["rm", "sub/../sub/file.txt"]).current_dir(repo).assert().success();
}
