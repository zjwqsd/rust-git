#!/bin/bash
set -e

echo "📁 清理旧目录"
rm -rf test_merge_debug
mkdir test_merge_debug
cp rust-git test_merge_debug/
cd test_merge_debug

echo "📦 初始化仓库并提交 delete.txt"
./rust-git init
./rust-git checkout -b main
echo "hello world" > delete.txt
./rust-git add delete.txt
./rust-git commit -m "add file"

echo "🌿 创建 temp 分支并删除 delete.txt"
./rust-git branch temp
./rust-git checkout temp

# ✅ 重新 add 一次，确保 index 中包含 delete.txt
./rust-git add delete.txt
./rust-git rm delete.txt
./rust-git commit -m "remove file"

echo "⬅️ 回到 main 分支并合并 temp"
./rust-git checkout main
./rust-git merge temp

echo "🔍 获取 merge 后提交哈希"
MERGE_HASH=$(cat .mygit/refs/heads/main | tr -d '\n')
echo "Merge Commit: $MERGE_HASH"

echo "🔍 获取 merge commit 的 tree 哈希"
OBJ_DIR=".mygit/objects/${MERGE_HASH:0:2}"
OBJ_FILE="${OBJ_DIR}/${MERGE_HASH:2}"
TREE_HASH=$(grep '^tree' "$OBJ_FILE" | awk '{print $2}')
echo "Tree: $TREE_HASH"

echo "📄 Tree 内容："
TREE_OBJ_DIR=".mygit/objects/${TREE_HASH:0:2}"
TREE_OBJ_FILE="${TREE_OBJ_DIR}/${TREE_HASH:2}"
cat "$TREE_OBJ_FILE"

echo ""
if grep -q 'delete.txt' "$TREE_OBJ_FILE"; then
    echo "❌ 错误：merge 后 tree 仍然包含 delete.txt"
    exit 1
else
    echo "✅ 正确：merge 后 tree 中不再包含 delete.txt"
fi
echo "✅ 测试通过：merge 后 tree 正常"