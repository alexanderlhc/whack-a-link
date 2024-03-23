pub const PRIME_32: u32 = 16777619;
pub const OFFSET_32: u32 = 2166136261;

pub trait FnvHash {
    fn hash(&self, data: &str) -> u32;
}

pub struct Fnv {
    pub prime: u32,
    pub offset: u32,
}

impl FnvHash for Fnv {
    fn hash(&self, data: &str) -> u32 {
        let mut hash: u32 = self.offset;

        for byte in data.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(self.prime);
        }
        hash as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static FNV: Fnv = Fnv {
        prime: PRIME_32 as u32,
        offset: OFFSET_32 as u32,
    };

    #[test]
    fn test_fnv_hash() {
        todo!("Find a correct test case to match against");
        let data = "Hello World";
        let hashed = FNV.hash(data);
        assert_eq!(hashed, 123);
    }
}
