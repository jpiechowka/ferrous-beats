use crate::handlers::errors::ServerError;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::path::Path;
use tracing::{debug, instrument, warn};

#[derive(Debug, Serialize)]
pub struct YtDlpStatusResponse {
    exists: bool,
    path: String,
}

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_status(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<YtDlpStatusResponse>), ServerError> {
    debug!("Handling checking of yt-dlp status");

    let dlp_dir = &app_state.config.server_settings.dlp_download_dir;
    let exists = Path::new(dlp_dir).is_dir();

    let status = YtDlpStatusResponse {
        exists,
        path: dlp_dir.to_string(),
    };

    if !exists {
        warn!("yt-dlp download directory does not exist: {}", dlp_dir);
    }

    Ok((StatusCode::OK, Json(status)))
}
