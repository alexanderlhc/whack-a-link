use crate::fnv::{Fnv, FnvHash, OFFSET_32, PRIME_32};

pub struct ShortCode(pub String);

pub trait Hash {
    fn compress(&self) -> String;
}

const FNV: Fnv = Fnv {
    prime: PRIME_32,
    offset: OFFSET_32,
};

impl Hash for ShortCode {
    fn compress(&self) -> String {
        let hashed = FNV.hash1a(&self.0);
        format!("{:x}", hashed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_code() {
        let short_code = ShortCode("hello".to_string());
        assert_eq!(short_code.compress(), "4f9f2cab");
    }

    #[test]
    fn handles_empty_string() {
        let short_code = ShortCode("".to_string());
        assert_eq!(short_code.compress(), "811c9dc5");
    }

    #[test]
    fn handles_unicode() {
        let short_code = ShortCode("😀".to_string());
        assert_eq!(short_code.compress(), "33a29608");
    }

    #[test]
    fn handles_long_string() {
        let short_code = ShortCode(
            "hello world, how are you today? tired of running meaningless tests?".to_string(),
        );
        assert_eq!(short_code.compress(), "e21c971c");
    }

    #[test]
    fn handles_odd_characters() {
        let short_code = ShortCode("!#¤%&/()=?".to_string());
        assert_eq!(short_code.compress(), "7c85c4ca");
    }
}
