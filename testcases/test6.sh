# 当前目录位于 testcases
 # 创建一个空目录 ad_test1
 mkdir ad_test1
 # 拷贝 rust-git 到 ad_test1 目录
cp rust-git ad_test1/
 # 进入 ad_test1 目录
cd ad_test1
 # 执行 rust-git init
 ./rust-git init
 # 创建 main 分支并切换到 main 分支
./rust-git checkout -b main
 # 创建 test.txt 文件并添加内容
echo "main分支修改内容" > test.txt
 # 添加并提交 test.txt
 ./rust-git add .
 ./rust-git commit -m "main"
 # 创建 temp1 和temp 2分支
./rust-git branch temp1
 ./rust-git branch temp2
 # 切换到 temp1 分支
./rust-git checkout temp1
 # 修改 test.txt 文件并添加内容
echo "temp1分支修改内容" > test.txt
 # 添加并提交 test.txt
 ./rust-git add .
 ./rust-git commit -m "temp1"
 # 切换回 main 分支
 ./rust-git checkout main
 # 切换到 temp2 分支
./rust-git checkout temp2
 # 修改 test.txt 文件并添加内容
echo "temp2分支修改内容" > test.txt
 # 添加并提交 test.txt
 ./rust-git add .
 ./rust-git commit -m "temp2"
 # 合并 test 分支并检查是否提示冲突
if ./rust-git merge temp1 2>&1 | grep -q "Merge conflict in test.txt: 1"; then
 echo "Conflict detected correctly"
 else
 echo "Conflict not detected"
 exit 1
 fi