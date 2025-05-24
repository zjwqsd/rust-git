# 当前目录位于 testcases
 # 创建一个空目录 ad_test3
 rm -r ad_test3
 mkdir ad_test3
 # 拷贝 rust-git 到 ad_test3 目录
cp rust-git ad_test3/
 # 进入 ad_test3 目录
cd ad_test3
 # 执行 rust-git init
./rust-git init
# 创建 main 分支并切换到 main 分支
./rust-git checkout -b main
 # 创建 main.txt 文件并添加内容

echo "main分支创建" > main.txt
  # 添加并提交 test.txt
./rust-git add .
commit_hash1=$(./rust-git commit -m "update main" 2>&1)

# 创建 temp 分支
./rust-git branch temp

# 切换到 temp 分支
./rust-git checkout temp
 # 创建 temp.txt 文件并添加内容
echo "test分支创建" > temp.txt
 # 添加并提交 temp.txt
./rust-git add .
commit_hash2=$(./rust-git commit -m "update temp" 2>&1)
# 切换回 main 分支
./rust-git checkout main
 # 合并 temp 分支
 ./rust-git merge temp
  # 删除 temp 分支
 ./rust-git branch -d temp

  # 检查当前目录下是否存在 test.txt 和 temp.txt 文件
 if [ -f "main.txt" ] && [ -f "temp.txt" ]; then
  echo "Both test.txt and temp.txt exist in the working directory"
else
    echo "Files are missing in the working directory"
    exit 1
fi

 # 检查 .git/refs/heads 目录下是否只存在 main 分支的引用文件
if [ -f ".git/refs/heads/main" ] && [ ! -f ".git/refs/heads/temp" ]; then
  echo "Only main branch reference exists in .git/refs/heads"
else
  echo "Branch references other than main exist in .git/refs/heads"
  exit 1
fi

echo "Advanced Test 3 passed: Branch temp is deleted and temp.txt is merged into main"

