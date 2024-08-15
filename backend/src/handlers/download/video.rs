use serde::Serialize;

#[derive(Serialize)]
pub struct DownloadVideoResponse {
    exists: bool,
    path: String,
}
