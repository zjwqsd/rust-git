# 当前目录位于 testcases
 # 创建一个空目录 test5
rm -rf test5
mkdir test5
 # 拷贝 rust-git 到 test5 目录
cp rust-git test5/
 # 进入 test5 目录
cd test5
 # 执行 rust-git init
 ./rust-git init
 # 创建 main 分支并切换到 main 分支
./rust-git checkout -b main
 # 创建 delete.txt 文件并添加内容
echo "Delete me" > delete.txt
 # 添加并提交 delete.txt
 ./rust-git add .
 ./rust-git commit -m "add file"
 # 创建 temp 分支
./rust-git branch temp
 # 切换到 temp 分支
./rust-git checkout temp
 # 删除 delete.txt 文件
./rust-git rm delete.txt
 ./rust-git commit -m "delete file"
  # 切换回 main 分支
./rust-git checkout main
 # 合并 temp 分支
./rust-git merge temp
 # 检查当前目录下是否不存在 delete.txt 文件
if [ ! -f "delete.txt" ]; then
 echo "Test 5 passed: git rm succeeded"
 else
 echo "Test 5 failed"
 exit 1
fi