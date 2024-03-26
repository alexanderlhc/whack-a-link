use crate::fnv::{Fnv, FnvHash, OFFSET_32, PRIME_32};

pub struct Link(pub String);

pub trait Hash {
    fn compress(&self) -> String;
}

const FNV: Fnv = Fnv {
    prime: PRIME_32,
    offset: OFFSET_32,
};

impl Hash for Link {
    fn compress(&self) -> String {
        let hashed = FNV.hash1a(&self.0);
        format!("{:x}", hashed)
    }
}
