use sha1::{Digest, Sha1};

pub fn hash(object: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();

    hasher.update(object);

    let hash_result = hasher.finalize();
    let vec = hash_result.to_vec();

    vec
}

pub fn hash_to_string(hash: &[u8]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("")
}