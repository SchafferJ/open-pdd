use std::fmt::Write;
use md5::{Digest, Md5};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time error occurred")
}

pub fn now_as_secs() -> u64 {
    now().as_secs()
}

pub fn now_as_millis() -> u128 {
    now().as_millis()
}

pub fn md5_str(src: &str) -> String {
    md5(src.as_bytes())
        .iter()
        .fold(String::new(), |mut s, byte|{
            let _ = write!(s, "{byte:02X}");
            s
        })
}

#[inline]
fn md5(src: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(src);
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hasher.finalize()[..16]);
    bytes
}