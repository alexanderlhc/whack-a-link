pub const PRIME_32: u32 = 16777619;
pub const OFFSET_32: u32 = 2166136261;

pub trait FnvHash {
    fn hash1a(&self, data: &str) -> u32;
    fn hash1(&self, data: &str) -> u32;
}

pub struct Fnv {
    pub prime: u32,
    pub offset: u32,
}

impl FnvHash for Fnv {
    fn hash1(&self, data: &str) -> u32 {
        let mut hash: u32 = self.offset;

        for byte in data.bytes() {
            hash = hash.wrapping_mul(self.prime);
            hash ^= byte as u32;
        }
        hash as u32
    }
    fn hash1a(&self, data: &str) -> u32 {
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
        let data = "Hello World";
        let hashed = FNV.hash1(data);
        let hashed_hex = format!("{:x}", hashed);
        assert_eq!(hashed_hex, "1282a4ef");
    }

    #[test]
    fn test_fnv1a() {
        let data = "Hello World";
        let hashed = FNV.hash1a(data);
        let hashed_hex = format!("{:x}", hashed);
        assert_eq!(hashed_hex, "b3902527");
    }
}
