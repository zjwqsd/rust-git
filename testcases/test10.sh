#!/bin/bash
# 测试 rust-git 的 branch 和 checkout 功能
# 创建⼀个空⽬录 test5
mkdir test5
# 拷⻉ rust-git 到 test5 ⽬录
cp ./rust-git test5/
# 进入 test5 ⽬录
cd test5
# 执⾏ rust-git init
./rust-git init
# 创建文件 test.txt 并添加内容
echo "1" > test.txt
# 执⾏ rust-git add 和 rust-git commit
./rust-git add test.txt
hash=$(/bin/bash -c './rust-git commit -m "Initial commit" 2>&1')
# 创建 feature 和 hotfix 分⽀。可能会同时创建多个分⽀
./rust-git branch feature
./rust-git branch hotfix
# 切换到 feature 分⽀
./rust-git checkout feature
# 验证分⽀切换是否成功。请注意，⼀定要及时更新.git/HEAD内容，否则直接报错退出。
if ! grep -q "ref: refs/heads/feature" ".git/HEAD"; then
echo "Failed to switch to feature branch"
exit 1
fi
# 切换到 hotfix 分⽀
./rust-git checkout hotfix
# 验证分⽀切换是否成功
if ! grep -q "ref: refs/heads/hotfix" ".git/HEAD"; then
echo "Failed to switch to hotfix branch"
exit 1
fi
# 输出成功信息
echo "Success!"
# 请思考，如果切换到不存在的分⽀，应该如何处理。我们假设默认的分⽀是master分⽀哦。
# 尝试切换到不存在的分⽀
message=$(./rust-git checkout non-existent 2>&1)
# 验证当前分⽀是否仍为 master
 if grep -q "ref: refs/heads/master" ".git/HEAD"; then
echo "Success!"
 fi