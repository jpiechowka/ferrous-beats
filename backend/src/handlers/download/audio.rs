use serde::Serialize;

#[derive(Serialize)]
pub struct DownloadAudioResponse {
    exists: bool,
    path: String,
}
