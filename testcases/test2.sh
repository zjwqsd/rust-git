#!/bin/bash
 # 当前目录位于 testcases
 # 创建一个空目录 test2
mkdir test2
 # 拷贝 rust-git 到 test2 目录
cp rust-git test2/
 # 进入 test2 目录
cd test2
 # 执行 rust-git init
 ./rust-git init
 
 # 创建文件 test.txt 并添加内容
echo "Hello, Rust!" > test.txt

# 检查 test.txt 文件是否存在
if [ -f "test.txt" ]; then
 echo "test.txt exists"
 else
 echo "test.txt does not exist"
 exit 1
 fi
 # 执行 git add 和 git commit
 ./rust-git add test.txt
 ./rust-git commit -m "Initial commit"
 
  # 验证 .git/objects 目录是否不为空
if [ "$(ls -A .git/objects)" ]; then
 echo ".git/objects directory is not empty"
 else
 echo ".git/objects directory is empty"
 exit 1
 fi
 # 验证 .git/refs/heads/main 文件是否存在且不为空
if [ -s ".git/refs/heads/master" ]; then
 echo ".git/refs/heads/master exists and is not empty"
 else
 echo ".git/refs/heads/master does not exist or is empty"
 exit 1
 fi
 echo "Test 2 passed: git add and git commit succeeded"