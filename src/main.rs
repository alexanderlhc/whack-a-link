use whack_a_link::fnv::{Fnv, FnvHash, OFFSET_32, PRIME_32};

fn main() {
    let fnv = Fnv {
        prime: PRIME_32,
        offset: OFFSET_32,
    };

    let data = "hello";
    let hash = fnv.hash1a(data);
    println!("Hash for {} is {}", data, hash);
}
