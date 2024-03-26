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
        format!("{}/{}", self.base_url.as_ref(), self.short.compress())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::url::Url;
    #[test]
    fn test_short_url() {
        let base_url = Url::parse("https://www.rust-lang.org").unwrap();
        let short = ShortCode("hello".to_string());
        let short_url = ShortUrl::new(base_url, short);
        assert_eq!(short_url.to_url(), "https://www.rust-lang.org/4f9f2cab");
    }
}
