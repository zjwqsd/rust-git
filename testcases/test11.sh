#!/bin/bash
# 测试 rust-git 的 merge 功能
# 创建⼀个空⽬录 test8
rm -r test11
mkdir test11
# 拷⻉ rust-git 到 test8 ⽬录
cp ./rust-git test11/
# 进入 test8 ⽬录
cd test11
# 执⾏ rust-git init
./rust-git init
# 创建 main 分⽀并切换到 main 分⽀
./rust-git checkout -b main
# 创建 main.txt 文件并添加内容
echo "Main" > main.txt
# 添加并提交 main.txt
./rust-git add main.txt
hash=$(/bin/bash -c './rust-git commit -m "Add main.txt" 2>&1')
# 创建 test 分⽀
./rust-git branch test
# 切换到 test 分⽀
./rust-git checkout test
# 创建 test.txt 文件并添加内容
echo "Test" > test.txt
# 添加并提交 test.txt
./rust-git add test.txt
hash1=$(/bin/bash -c './rust-git commit -m "Add test.txt" 2>&1')
# 切换回 main 分⽀
./rust-git checkout main
 # 合并 test 分⽀。请注意，合并分⽀中的不同文件的情景。
 merge_output=$(/bin/bash -c './rust-git merge test 2>&1')
 # 验证 main 分⽀是否包含 main.txt 和 test.txt 文件。除了文件，还可能是代码文件。
 if [ -f "main.txt" ] && [ -f "test.txt" ]; then
 # 如果两个文件都存在，输出成功信息
 echo "Success!"
  else
 echo "Files do not exist in the main branch!"
 exit 1
 fi

## 如果合并的是Rust代码文件，我们还会⽤rustc编译，并检查程序执⾏结果。
## 尝试编译 read_and_modify.rs
#rustc read_and_modify.rs || { echo "Failed to compile read_and_modify.rs"; exit 1;}
# # 执⾏编译后的程序
# ./read_and_modify > code_file.txt
# content=$(cat code_file.txt)
# echo "$content"
# # 假设当前分⽀是main，我们还可能测试合并多个分⽀。merge操作如果冲突则会输出冲突信息，⾏号。
# # 合并 temp1 分⽀
# content1=$(./rust-git merge temp1 2>&1)
# # 合并 temp2 分⽀
# content2=$(./rust-git merge temp2 2>&1)