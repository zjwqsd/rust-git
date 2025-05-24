#!/bin/bash
# 测试 rust-git 的 rm 功能
# 创建⼀个空⽬录 test12
rm -r test12
mkdir test12
# 拷⻉ rust-git 到 test12 ⽬录
cp ./rust-git test12/
# 进入 test12 ⽬录
cd test12
# 执⾏ rust-git init
./rust-git init
# 创建 main 分⽀并切换到 main 分⽀
./rust-git checkout -b main

 # 创建 delete.txt 文件并添加内容
echo "Del" > delete.txt
# 添加并提交文件
./rust-git add delete.txt
hash=$(/bin/bash -c './rust-git commit -m "Add delete.txt" 2>&1')
# 创建 temp 分⽀
./rust-git branch temp
# 切换到 temp 分⽀
./rust-git checkout temp
# 修改文件
echo "Modify" > delete.txt
# 添加并提交修改
./rust-git add delete.txt
hash1=$(/bin/bash -c './rust-git commit -m "Modify delete.txt" 2>&1')
# 删除文件
./rust-git rm delete.txt
# 提交删除操作
hash2=$(/bin/bash -c './rust-git commit -m "Delete file" 2>&1')
# 切换回 main 分⽀
./rust-git checkout main
# 合并 temp 分⽀
merge_output=$(/bin/bash -c './rust-git merge temp 2>&1')
# 检查当前⽬录下是否不存在 delete.txt 文件
if [ ! -f "delete.txt" ]; then
echo "Success!"
 else
echo "Failed to remove the file!"
exit 1
fi