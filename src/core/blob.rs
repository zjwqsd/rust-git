use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::utils::hash::sha1_hash;
use crate::core::config::GIT_DIR;
/// 将指定路径的文件内容写入 Git 风格的对象存储中，并返回该内容的 SHA-1 哈希值。
///
/// 该函数会执行以下步骤：
/// 1. 读取目标文件内容；
/// 2. 计算其 SHA-1 哈希；
/// 3. 将内容写入 `.mygit/objects/xx/yyyy...` 路径中；
/// 4. 如果该对象已存在则不会重复写入。
///
/// # 参数
///
/// - `path`: 目标文件的路径。
///
/// # 返回
///
/// 返回该文件内容的 `SHA-1` 哈希字符串。
///
/// # 错误
///
/// 如果文件读取失败、目录无法创建，或文件写入失败，会返回相应的 I/O 错误。
///
/// # 示例
///
/// 假设你有一个文件 `example.txt` 内容为 `hello`：
///
/// ```
/// use std::path::Path;
/// use mygit::write_blob;
///
/// let path = Path::new("example.txt");
/// let hash = write_blob(&path).unwrap();
/// println!("文件哈希: {}", hash);
/// ```
///
pub fn write_blob(path: &Path) -> io::Result<String> {
    let content = fs::read(path)?;
    let hash = sha1_hash(&content);
    let (dir, file) = hash.split_at(2);

    let object_dir = &*GIT_DIR.join("objects").join(dir);
    if !object_dir.exists() {
        fs::create_dir_all(&object_dir)?;
    }

    let object_path = object_dir.join(file);
    if !object_path.exists() {
        let mut file = fs::File::create(&object_path)?;
        file.write_all(&content)?;
    }

    Ok(hash)
}


