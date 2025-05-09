use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::utils::hash::sha1_hash;

pub fn write_blob(path: &Path) -> io::Result<String> {
    let content = fs::read(path)?;
    let hash = sha1_hash(&content);
    let (dir, file) = hash.split_at(2);

    let object_dir = Path::new(".mygit").join("objects").join(dir);
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


/// 删除指定哈希的 blob 对象（简化逻辑）
pub fn remove_blob(hash: &str) -> io::Result<()> {
    let (dir, file) = hash.split_at(2);
    let blob_path = Path::new(".mygit/objects").join(dir).join(file);
    if blob_path.exists() {
        fs::remove_file(blob_path)?;
    }
    Ok(())
}