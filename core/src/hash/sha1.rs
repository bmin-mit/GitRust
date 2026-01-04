use sha1::{Digest, Sha1};

pub fn hash(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut out = [0u8; 20];
    out.copy_from_slice(&result[..]);

    out
}
