pub const PRIME_32: u32 = 16777619;
pub const OFFSET_32: u32 = 2166136261;

pub trait FnvHash {
    fn hash(&self, data: &str) -> u64;
}

pub struct Fnv {
    pub prime: u64,
    pub offset: u64,
}

impl FnvHash for Fnv {
    fn hash(&self, data: &str) -> u64 {
        let mut hash: u64 = self.offset;

        for byte in data.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(self.prime);
        }
        hash as u64
    }
}
