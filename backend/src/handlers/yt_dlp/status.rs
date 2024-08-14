use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::path::Path;
use tracing::debug;

#[derive(Serialize)]
pub struct YtDlpStatusResponse {
    exists: bool,
    path: String,
}

pub async fn yt_dlp_status(
    State(app_state): State<AppState>,
) -> (StatusCode, Json<YtDlpStatusResponse>) {
    debug!("Handling checking of yt-dlp status");

    let dlp_dir = &app_state.config.server_settings.dlp_download_dir;
    let exists = Path::new(dlp_dir).is_dir();

    let status = YtDlpStatusResponse {
        exists,
        path: dlp_dir.to_string(),
    };

    (StatusCode::OK, Json(status))
}
