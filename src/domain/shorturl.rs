use crate::domain::shortcode::Hash;

use super::{shortcode::ShortCode, url::Url};

pub struct ShortUrl {
    base_url: Url,
    short: ShortCode,
}

impl ShortUrl {
    pub fn new(base_url: Url, short: ShortCode) -> Self {
        ShortUrl { base_url, short }
    }
}

impl ShortUrl {
    pub fn to_url(&self) -> String {
        format!(
            "{}/{}",
            self.base_url.without_trailing_slash(),
            self.short.compress()
        )
    }
}

impl ShortUrl {
    pub fn from_shortcode(base_url: &str, shortcode: &str) -> Url {
        todo!()
        // Url::parse(&format!("{}/{}", base_url, shortcode)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::url::Url;

    fn make_shortcode() -> ShortCode {
        ShortCode(String::from("hello"))
    }

    #[test]
    fn test_short_url() {
        let base_url = Url::parse("https://www.rust-lang.org").unwrap();
        let short_url = ShortUrl::new(base_url, make_shortcode());
        assert_eq!(short_url.to_url(), "https://www.rust-lang.org/4f9f2cab");
    }

    #[test]
    fn test_short_url_with_path() {
        let base_url = Url::parse("https://www.rust-lang.org/path/to/something").unwrap();
        let short_url = ShortUrl::new(base_url, make_shortcode());
        assert_eq!(
            short_url.to_url(),
            "https://www.rust-lang.org/path/to/something/4f9f2cab"
        );
    }

    #[test]
    fn test_short_url_adds_slash() {
        let base_url = Url::parse("https://www.rust-lang.org/").unwrap();
        let short_url = ShortUrl::new(base_url, make_shortcode());
        assert_eq!(short_url.to_url(), "https://www.rust-lang.org/4f9f2cab");
    }
}
