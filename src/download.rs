//! Download model.

use super::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadState { Pending, Active, Paused, Completed, Failed }

#[derive(Debug, Clone)]
pub struct Download {
    pub url: Url,
    pub output_path: String,
    pub state: DownloadState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download() {
        let d = Download { url: Url { scheme: crate::UrlScheme::Http, host: "example.com".into(), port: 80, path: "/file".into() }, output_path: "/tmp/file".into(), state: DownloadState::Pending };
        assert_eq!(d.state, DownloadState::Pending);
    }
}
