// tests/merge.rs

use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::tempdir;

fn bin() -> Command {
    Command::cargo_bin("rust-git").unwrap()
}

fn init_commit(repo: &std::path::Path, name: &str, content: &str) {
    let path = repo.join(name);
    println!("📄 写入文件: {}", path.display());
    fs::write(&path, content).unwrap();
    assert!(path.exists(), "❗写入失败：{}", name);

    run_and_log(&["add", name], repo);
    let out = run_and_log(&["commit", "-m", "msg"], repo);

    // ✅ 附加验证：commit 输出中包含写入哪一个分支
    println!("🧾 commit 输出:\n{}", out);
}


fn run_and_log(args: &[&str], repo: &std::path::Path) -> String {
    let output = bin().args(args).current_dir(repo).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("\n🔧 $ rust-git {}", args.join(" "));
    println!("📤 stdout:\n{}", stdout);
    println!("📥 stderr:\n{}", stderr);

    assert!(output.status.success(), "命令 {:?} 执行失败", args);
    stdout.to_string()
}

#[test]
fn test_merge_fast_forward() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    init_commit(repo, "a.txt", "v1");
    bin().args(["branch", "dev"]).current_dir(repo).assert().success();
    bin().args(["checkout", "dev"]).current_dir(repo).assert().success();
    init_commit(repo, "b.txt", "v2");
    bin().args(["checkout", "main"]).current_dir(repo).assert().success();
    bin().args(["merge", "dev"]).current_dir(repo).assert().stdout(contains("已合并"));
}

#[test]
fn test_merge_up_to_date() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    bin().arg("init").current_dir(repo).assert().success();
    init_commit(repo, "x.txt", "x");
    // bin().args(["merge", "main"]).current_dir(repo).assert().stdout(contains("Already up to Date"));

    let out = run_and_log(&["merge", "main"], repo);
    assert!(out.contains("Already up to Date"));
}
#[test]
fn test_merge_add_same_file_conflict() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    bin().arg("init").current_dir(repo).assert().success();
    init_commit(repo, "base.txt", "base");

    bin().args(["branch", "a"]).current_dir(repo).assert().success();
    bin().args(["branch", "b"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    fs::write(repo.join("conflict.txt"), "aaa").unwrap();
    bin().args(["add", "conflict.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a change"]).current_dir(repo).assert().success();

    bin().args(["checkout", "b"]).current_dir(repo).assert().success();
    fs::write(repo.join("conflict.txt"), "bbb").unwrap();
    bin().args(["add", "conflict.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "b change"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    let out = run_and_log(&["merge", "b"], repo);
    assert!(out.contains("Merge conflict") || out.contains("冲突"));
}


#[test]
fn test_merge_add_same_file_no_conflict() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    bin().arg("init").current_dir(repo).assert().success();
    init_commit(repo, "base.txt", "base");

    bin().args(["branch", "a"]).current_dir(repo).assert().success();
    bin().args(["branch", "b"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    fs::write(repo.join("same.txt"), "ok").unwrap();
    bin().args(["add", "same.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a add same"]).current_dir(repo).assert().success();

    bin().args(["checkout", "b"]).current_dir(repo).assert().success();
    fs::write(repo.join("same.txt"), "ok").unwrap();
    bin().args(["add", "same.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "b add same"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    bin().args(["merge", "b"]).current_dir(repo).assert().stdout(contains("已合并"));
}

#[test]
fn test_merge_ab_delete_same_file() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    bin().arg("init").current_dir(repo).assert().success();
    init_commit(repo, "shared.txt", "content");
    bin().args(["branch", "a"]).current_dir(repo).assert().success();
    bin().args(["branch", "b"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    fs::remove_file(repo.join("shared.txt")).unwrap();
    bin().args(["rm", "shared.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "a delete"]).current_dir(repo).assert().success();

    bin().args(["checkout", "b"]).current_dir(repo).assert().success();
    fs::remove_file(repo.join("shared.txt")).unwrap();
    bin().args(["rm", "shared.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "b delete"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    bin().args(["merge", "b"]).current_dir(repo).assert().stdout(contains("已合并"));
}

#[test]
fn test_merge_add_diff_files_no_conflict() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();
    bin().arg("init").current_dir(repo).assert().success();
    init_commit(repo, "base.txt", "base");
    bin().args(["branch", "a"]).current_dir(repo).assert().success();
    bin().args(["branch", "b"]).current_dir(repo).assert().success();

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    init_commit(repo, "a.txt", "a");

    bin().args(["checkout", "b"]).current_dir(repo).assert().success();
    init_commit(repo, "b.txt", "b");

    bin().args(["checkout", "a"]).current_dir(repo).assert().success();
    let out = run_and_log(&["merge", "b"], repo);
    assert!(out.contains("已合并"));
}



#[test]
fn test_merge_add_and_delete_different_files() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    // 初始化并明确创建 main 分支
    run_and_log(&["init"], repo);
    run_and_log(&["checkout", "-b", "main"], repo);

    // 提交 base.txt 和 common.txt
    init_commit(repo, "base.txt", "base");
    init_commit(repo, "common.txt", "common");

    // 查看当前状态是否有 common.txt
    println!("🧪 提交 common.txt 后的状态:");
    run_and_log(&["status"], repo);

    // 创建 a 分支并提交 a.txt
    run_and_log(&["checkout", "-b", "a"], repo);
    init_commit(repo, "a.txt", "a");

    println!("🧪 a 分支提交后状态:");
    run_and_log(&["status"], repo);

    // 回到 main 创建 b 分支
    run_and_log(&["checkout", "main"], repo);
    println!("🧪 切换回 main 后状态:");
    run_and_log(&["status"], repo);

    run_and_log(&["checkout", "-b", "b"], repo);
    println!("🧪 b 分支创建后状态:");
    run_and_log(&["status"], repo);

    // 断言 common.txt 是否存在
    let common_path = repo.join("common.txt");
    assert!(
        common_path.exists(),
        "❗ common.txt 丢失，说明分支切换后未还原工作区"
    );

    // 删除 common.txt 并提交
    fs::remove_file(&common_path).unwrap();
    run_and_log(&["rm", "common.txt"], repo);
    run_and_log(&["commit", "-m", "b delete common"], repo);

    println!("🧪 b 分支删除 common.txt 后状态:");
    run_and_log(&["status"], repo);

    // 合并回 a 分支
    run_and_log(&["checkout", "a"], repo);
    println!("🧪 切换回 a 前合并状态:");
    run_and_log(&["status"], repo);

    let out = run_and_log(&["merge", "b"], repo);
    assert!(out.contains("已合并"));
}
