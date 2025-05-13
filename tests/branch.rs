// tests/branch.rs

use assert_cmd::Command;
// use predicates::str::contains;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}

fn assert_branch_exists(repo: &Path, name: &str) {
    let path = repo.join(".mygit/refs/heads").join(name);
    assert!(path.exists(), "分支 {} 不存在", name);
}

fn assert_head_points_to(repo: &Path, branch: &str) {
    let head = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap();
    let expected = format!("ref: refs/heads/{}", branch);
    assert_eq!(head.trim(), expected, "HEAD 没有指向 {}", branch);
}


/// 获取当前 HEAD 指向的 commit hash，无论是分支还是 detached 状态。
pub fn get_current_commit_hash(repo: &Path) -> String {
    let head_path = repo.join(".mygit/HEAD");

    let head_content = fs::read_to_string(&head_path)
        .unwrap_or_default()
        .trim()
        .to_string();

    if head_content.starts_with("ref: ") {
        let ref_path = repo.join(".mygit").join(
            head_content.trim_start_matches("ref: ").trim()
        );
        fs::read_to_string(ref_path)
            .unwrap_or_default()
            .trim()
            .to_string()
    } else {
        head_content
    }
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

    assert_branch_exists(repo, "main");
    assert_branch_exists(repo, "dev");
    assert_head_points_to(repo, "main");
}

#[test]
fn test_branch_list_in_detach() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "first commit"]).current_dir(repo).assert().success();

    let commit_hash = get_current_commit_hash(repo);
    println!("📦 当前 commit_hash: {}", commit_hash);

    bin().args(["checkout", &commit_hash]).current_dir(repo).assert().success();

    let new_head = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    assert_eq!(new_head, commit_hash, "HEAD 没变成 detached 状态");
}

#[test]
fn test_branch_create_in_detach() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "first commit"]).current_dir(repo).assert().success();
    // let commit_hash = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    let commit_hash = get_current_commit_hash(repo);
    println!("📦 当前 commit_hash: {}", commit_hash);
    bin().args(["checkout", &commit_hash]).current_dir(repo).assert().success();
    bin().args(["branch", "new-branch"]).current_dir(repo).assert().success();

    assert_branch_exists(repo, "new-branch");
}

#[test]
fn test_branch_after_add() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "1").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["branch", "dev"]).current_dir(repo).assert().success();

    assert_branch_exists(repo, "dev");
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

    assert_branch_exists(repo, "x");
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

    assert_branch_exists(repo, "lostfile");
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

    assert_branch_exists(repo, "resurrect");
}

// 在 detached HEAD 下提交新的 commit，不应影响已有分支
#[test]
fn test_commit_in_detached_head_does_not_change_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    // 初始化并提交
    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "first"]).current_dir(repo).assert().success();

    let first_commit = get_current_commit_hash(repo);
    bin().args(["checkout", &first_commit]).current_dir(repo).assert().success();

    // 修改并提交
    fs::write(repo.join("file.txt"), "v2").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "second"]).current_dir(repo).assert().success();

    let second_commit = get_current_commit_hash(repo);
    assert_ne!(first_commit, second_commit, "应生成新提交");

    // 切回 main，看它是否还是指向 first commit（未被更新）
    bin().args(["checkout", "main"]).current_dir(repo).assert().success();
    let current = get_current_commit_hash(repo);
    assert_eq!(current, first_commit, "main 分支不应指向 detached 中的新提交");
}

//detached HEAD 下切换其他分支，应正常重设 HEAD
#[test]
fn test_checkout_branch_from_detached_head() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("f.txt"), "1").unwrap();
    bin().args(["add", "f.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "c1"]).current_dir(repo).assert().success();

    let hash = get_current_commit_hash(repo);
    bin().args(["checkout", &hash]).current_dir(repo).assert().success();

    bin().args(["branch", "dev"]).current_dir(repo).assert().success();
    bin().args(["checkout", "dev"]).current_dir(repo).assert().success();

    // HEAD 应该是 symbolic ref
    let head = std::fs::read_to_string(repo.join(".mygit/HEAD")).unwrap();
    assert_eq!(head.trim(), "ref: refs/heads/dev");
}

#[test]
fn test_create_invalid_branch_in_detached_should_fail() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "hi").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "m"]).current_dir(repo).assert().success();

    let hash = get_current_commit_hash(repo);
    bin().args(["checkout", &hash]).current_dir(repo).assert().success();

    let result = bin()
        .args(["branch", "bad:name"])
        .current_dir(repo)
        .output()
        .expect("should run");

    assert!(!result.status.success(), "应拒绝非法分支名");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("无效的分支名") || stderr.contains("invalid"),
        "应输出错误信息，实际输出: {}",
        stderr
    );
}


