//! URL model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlScheme { Http, Https, Ftp }

#[derive(Debug, Clone)]
pub struct Url {
    pub scheme: UrlScheme,
    pub host: String,
    pub port: u16,
    pub path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let u = Url { scheme: UrlScheme::Https, host: "example.com".into(), port: 443, path: "/".into() };
        assert_eq!(u.port, 443);
    }
}
