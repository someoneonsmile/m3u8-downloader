use std::fmt::Write;

use sha2::Digest as Sha2Digest;
use sha2::Sha256;

/// 使用 SHA256 对字节序列生成确定性哈希值（hex 字符串）
/// 用于生成稳定的缓存目录路径，保证断点续传在不同 Rust 版本间兼容
pub fn hash(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    Sha2Digest::update(&mut hasher, bytes);
    Sha2Digest::finalize(hasher)
        .iter()
        .fold(String::with_capacity(64), |mut acc, b| {
            let _ = write!(acc, "{b:02x}");
            acc
        })
}
