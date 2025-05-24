use assert_cmd::Command;
// use predicates::str::contains;
use std::path::Path;
use std::fs;
use tempfile::tempdir;

fn bin() -> Command {
    Command::cargo_bin("rust-git").expect("binary build failed")
}
fn run_and_print(args: &[&str], dir: &Path) {
    let output = bin().args(args).current_dir(dir).output().expect("failed to run");

    println!("\n🔧 $ rust-git {}", args.join(" "));
    println!("📤 stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("📥 stderr:\n{}", String::from_utf8_lossy(&output.stderr));

    assert!(
        output.status.success(),
        "Command {:?} failed with status {}",
        args,
        output.status
    );
}
#[test]
fn test_basic_commit() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    // 初始化仓库
    bin().arg("init").current_dir(repo).assert().success();

    // 写入文件
    let file = repo.join("hello.txt");
    fs::write(&file, "hello world").unwrap();

    // 添加 & 提交
    bin().args(["add", "hello.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "initial commit"]).current_dir(repo).assert().success();

    // HEAD 文件应存在
    assert!(repo.join(".mygit/HEAD").exists());
}

#[test]
fn test_branch_and_checkout() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "main commit"]).current_dir(repo).assert().success();

    bin().args(["branch", "dev"]).current_dir(repo).assert().success();
    bin().args(["checkout", "dev"]).current_dir(repo).assert().success();

    fs::write(repo.join("file.txt"), "v2").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "dev commit"]).current_dir(repo).assert().success();

    bin().args(["checkout", "master"]).current_dir(repo).assert().success();

    let content = fs::read_to_string(repo.join("file.txt")).unwrap();
    assert!(content.contains("v1"));
}

#[test]
fn test_add_same_file_twice() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();

    // 初始化 + v1 内容
    bin().arg("init").current_dir(repo).assert().success();
    fs::write(repo.join("file.txt"), "v1").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "v1 commit"]).current_dir(repo).assert().success();

    // 修改为 v2
    fs::write(repo.join("file.txt"), "v2").unwrap();
    bin().args(["add", "file.txt"]).current_dir(repo).assert().success();
    bin().args(["commit", "-m", "v2 commit"]).current_dir(repo).assert().success();

    // 检查内容是否为 v2（正确），若是 v1 说明 index 累积问题
    let content = fs::read_to_string(repo.join("file.txt")).unwrap();
    println!("🔍 Final file.txt content: {:?}", content);
    assert!(content.contains("v2"));
}

#[test]
fn test_merge_no_conflict() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    run_and_print(&["init"], repo);
    fs::write(repo.join("file.txt"), "v1").unwrap();
    run_and_print(&["add", "file.txt"], repo);
    run_and_print(&["commit", "-m", "main commit"], repo);

    run_and_print(&["branch", "dev"], repo);
    run_and_print(&["checkout", "dev"], repo);
    fs::write(repo.join("file.txt"), "v2").unwrap();
    run_and_print(&["add", "file.txt"], repo);
    run_and_print(&["commit", "-m", "dev commit"], repo);

    run_and_print(&["checkout", "main"], repo);
    run_and_print(&["merge", "dev"], repo);

    let content = fs::read_to_string(repo.join("file.txt")).unwrap();
    println!("🔍 [merge test] file.txt content = {:?}", content);
    assert!(content.contains("v2"));
}

#[test]
fn test_merge_conflict() {
    let tmp = tempdir().unwrap();
    let repo = tmp.path();

    // 初始化仓库
    run_and_print(&["init"], repo);
    fs::write(repo.join("test.txt"), "line1\nline2\n").unwrap();
    run_and_print(&["add", "test.txt"], repo);
    run_and_print(&["commit", "-m", "initial"], repo);

    // 创建 temp1 分支并修改
    run_and_print(&["branch", "temp1"], repo);
    run_and_print(&["checkout", "temp1"], repo);
    fs::write(repo.join("test.txt"), "line1\nchange-from-temp1\n").unwrap();
    run_and_print(&["add", "test.txt"], repo);
    run_and_print(&["commit", "-m", "temp1 edit"], repo);

    // 回到 main，创建 temp2 并修改
    run_and_print(&["checkout", "master"], repo);
    run_and_print(&["branch", "temp2"], repo);
    run_and_print(&["checkout", "temp2"], repo);
    fs::write(repo.join("test.txt"), "line1\nchange-from-temp2\n").unwrap();
    run_and_print(&["add", "test.txt"], repo);
    run_and_print(&["commit", "-m", "temp2 edit"], repo);

    // 合并 temp1，期望产生冲突
    let output = bin()
        .args(["merge", "temp1"])
        .current_dir(repo)
        .output()
        .expect("merge failed");

    println!("\n🔧 $ rust-git merge temp1");
    println!("📤 stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("📥 stderr:\n{}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Merge conflict in test.txt"),
        "冲突信息未输出"
    );
    assert!(
        stdout.contains("❗ 冲突发生"),
        "未提示手动解决冲突"
    );
}

