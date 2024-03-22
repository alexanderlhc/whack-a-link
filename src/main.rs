mod fnv;
use fnv::Fnv;

use crate::fnv::{FnvHash, OFFSET_32, PRIME_32};

fn main() {
    let fnv = Fnv {
        prime: PRIME_32 as u64,
        offset: OFFSET_32 as u64,
    };

    let data = "hello";
    let hash = fnv.hash(data);
    println!("Hash for {} is {}", data, hash);
}
