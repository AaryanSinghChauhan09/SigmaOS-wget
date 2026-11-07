//! Progress model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState { Idle, Running, Paused, Completed, Failed }

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub state: ProgressState,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_progress() {
        let p = DownloadProgress { state: ProgressState::Running, bytes_downloaded: 512, total_bytes: 1024 };
        assert_eq!(p.bytes_downloaded, 512);
    }
}
