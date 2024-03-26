use whack_a_link::fnv::{Fnv, FnvHash, OFFSET_32, PRIME_32};

const FNV: Fnv = Fnv {
    prime: PRIME_32,
    offset: OFFSET_32,
};

fn main() {
    let data = "hello";
    let res = shorten(data);
    println!("Shortened: {}", res)
}

pub fn shorten(text: &str) -> String {
    let hashed = FNV.hash1a(text);
    println!("Hashed: {}", hashed);
    let hex = format!("{:x}", hashed);
    hex
}
