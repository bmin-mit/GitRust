use sha1::{Digest, Sha1};

pub fn hash(object: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();

    hasher.update(object);

    let hash_result = hasher.finalize();
    let vec = hash_result.to_vec();

    vec
}
