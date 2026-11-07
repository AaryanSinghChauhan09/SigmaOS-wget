//! HTTP request model.

use super::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod { Get, Post, Head, Put, Delete }

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: Url,
    pub headers: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_request() {
        let r = HttpRequest { method: HttpMethod::Get, url: Url { scheme: crate::UrlScheme::Http, host: "example.com".into(), port: 80, path: "/".into() }, headers: vec![] };
        assert_eq!(r.method, HttpMethod::Get);
    }
}
