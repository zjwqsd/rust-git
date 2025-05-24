#!/bin/bash
# 当前目录位于 testcases
rm -r test1
mkdir test1
 # 拷贝 rust-git 到 test1 目录
cp rust-git test1/
 # 进入 test1 目录
cd test1
 # 执行 rust-git init
 ./rust-git init
 # 验证 .git 目录是否存在且不为空
if [ -d ".git" ] && [ "$(ls -A .git)" ]; then
 echo "Test 1 passed: .git directory exists and is not empty"
 else
 echo "Test 1 failed: .git directory does not exist or is empty"
 exit 1
fi