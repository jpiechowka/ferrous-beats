use crate::handlers::errors::ServerError;
use crate::handlers::shared::model::media::MediaDownloadResponse;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use tracing::{debug, instrument};

#[derive(Debug, Deserialize)]
pub struct DownloadVideoRequest {
    video_url: String,
}

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_video_download(
    State(app_state): State<AppState>,
    Json(payload): Json<DownloadVideoRequest>,
) -> Result<(StatusCode, Json<MediaDownloadResponse>), ServerError> {
    debug!("Handling video download");

    todo!()
}
