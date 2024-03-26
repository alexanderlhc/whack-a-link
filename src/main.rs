use whack_a_link::domain::link::{Hash, Link};

fn main() {
    let data = Link("hello".to_string());
    println!("Shortened: {}", data.compress())
}
