use whack_a_link::domain::{shortcode::ShortCode, shorturl::ShortUrl, url::Url};

fn main() {
    let data = ShortCode("hello".to_string());
    let url = Url::parse("https://www.rust-lang.org");
    let short_url = ShortUrl::new(url.unwrap(), data);

    println!("{}", short_url.to_url());
}
