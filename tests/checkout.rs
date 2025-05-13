use assert_cmd::Command;
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
fn test_checkout_b_creates_and_switches_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "main").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "main commit"]).current_dir(repo).assert().success();

    bin().args(["checkout", "-b", "feature"]).current_dir(repo).assert().success();
    assert_branch_exists(repo, "feature");
    assert_head_points_to(repo, "feature");
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
    assert_head_points_to(repo, "dev");
}

#[test]
fn test_checkout_to_detached_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "a").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a"]).current_dir(repo).assert().success();

    let hash = get_current_commit_hash(repo);
    bin().args(["checkout", &hash]).current_dir(repo).assert().success();

    let new_head = fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    assert_eq!(new_head, hash, "HEAD 没变成 detached 状态");
}

#[test]
fn test_checkout_from_detached_to_branch() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("a.txt"), "a").unwrap();
    bin().args(["add", "a.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a"]).current_dir(repo).assert().success();

    let hash = get_current_commit_hash(repo);
    bin().args(["checkout", &hash]).current_dir(repo).assert().success();
    bin().args(["checkout", "main"]).current_dir(repo).assert().success();

    assert_head_points_to(repo, "main");
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

    let commit2 = get_current_commit_hash(repo);

    // 获取 commit2 的父提交（从对象中解析）
    let commit1 = {
        let (dir, file) = commit2.split_at(2);
        let path = repo.join(".mygit/objects").join(dir).join(file);
        let content = fs::read_to_string(path).unwrap();
        content
            .lines()
            .find(|l| l.starts_with("parent "))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string()
    };

    bin().args(["checkout", &commit2]).current_dir(repo).assert().success();
    println!("📦 切换到 commit2: {}", commit2);
    bin().args(["checkout", &commit1]).current_dir(repo).assert().success();
    println!("📦 切换到 commit1: {}", commit1);
    let new_head = fs::read_to_string(repo.join(".mygit/HEAD")).unwrap().trim().to_string();
    assert_eq!(new_head, commit1, "HEAD 应该指向 commit1");
}
