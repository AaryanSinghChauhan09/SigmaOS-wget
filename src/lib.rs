//! SigmaOS-wget — HTTP download client model.
//!
//! Models HTTP downloads: URLs, headers, progress tracking,
//! resume support, and file I/O. Deterministic and std-only.

pub mod url;
pub mod request;
pub mod download;
pub mod progress;

pub use url::{Url, UrlScheme};
pub use request::{HttpRequest, HttpMethod};
pub use download::{Download, DownloadState};
pub use progress::{DownloadProgress, ProgressState};
