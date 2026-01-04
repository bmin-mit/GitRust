use crate::hash::hash;
use hex::{FromHex, FromHexError, encode};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Oid {
    pub hash: [u8; 20],
}

impl Oid {
    pub fn new(hash: &[u8; 20]) -> Self {
        Oid {
            hash: hash.to_owned(),
        }
    }

    pub fn from_str(str: &str) -> Result<Self, FromHexError> {
        Ok(Oid::new(&<[u8; 20]>::from_hex(str)?))
    }

    pub fn from_data(data: &[u8]) -> Self {
        let hash_result = hash(data);
        Oid { hash: hash_result }
    }
}

impl Display for Oid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", encode(self.hash))
    }
}

impl AsRef<[u8]> for Oid {
    fn as_ref(&self) -> &[u8] {
        &self.hash
    }
}
