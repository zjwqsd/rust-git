use sha1::{Sha1, Digest};

pub fn sha1_hash(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}
