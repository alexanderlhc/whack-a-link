pub struct Url(pub String);

impl Url {
    pub fn parse(url: &str) -> Result<Url, &'static str> {
        match external_url::Url::parse(url) {
            Ok(_) => Ok(Url(url.to_string())),
            Err(_) => Err("Invalid URL"),
        }
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Url;

    #[test]
    fn test_parses_valid_url() {
        let test_url = "https://www.rust-lang.org";
        let url = Url::parse(test_url).unwrap();
        assert_eq!(url.as_ref(), test_url);
    }

    #[test]
    fn test_no_parse_incorrect_scheme() {
        let test_url = "hØÆ@)({ts:.org";
        let url = Url::parse(test_url);
        assert!(url.is_err());
    }
}
