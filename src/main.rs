use whack_a_link::domain::{
    shortcode::{Hash, ShortCode},
    url::Url,
};

fn main() {
    let data = ShortCode("hello".to_string());
    let url = Url::parse("https://www.rust-lang.org");

    match url {
        Ok(u) => {
            println!("URL: {}", u.as_ref());
            println!("Shortened: {}", data.compress());
        }
        Err(e) => println!("Error: {}", e),
    }
}
