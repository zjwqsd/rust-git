# 当前目录位于 testcases
 # 创建 large_file.bin
 dd if=/dev/zero of=large_file.bin bs=1M count=10
 # 创建一个空目录 ad_test2
 rm -rf ad_test2
 mkdir ad_test2
 # 拷贝 rust-git到ad_test2 目录
cp rust-git ad_test2/
 # 进入 ad_test2 目录
cd ad_test2
# 执行 rust-git init
 ./rust-git init
 # 创建 main 分支并切换到 main 分支
./rust-git checkout -b main
 # 拷贝large_file.bin 到 ad_test2 目录
cp ../large_file.bin ./
# 添加 large_file.bin
 ./rust-git add large_file.bin
 # 提交 large_file.bin
 commit_hash=$(/usr/bin/git commit -m "Add large file")
 # 检查提交是否成功
if [ -z "$commit_hash" ]; then
 echo "Commit hash not found"
 exit 1
 fi
 echo "Committed changes: $commit_hash"
 # 提取 large_file.bin 的 SHA-1 哈希值
file_hash=$(/usr/bin/git hash-object large_file.bin)
 # 检查 .mygit/objects 目录下是否存在对应的对象文件
object_dir=".mygit/objects/${file_hash:0:2}"
 object_file="$object_dir/${file_hash:2}"
 if [ -d "$object_dir" ] && [ -f "$object_file" ]; then
 echo "Object file $object_file exists"
 else
 echo "Object file $object_file does not exist"
 exit 1
 fi
 echo "Advanced Test 2 passed: Large file is correctly stored"