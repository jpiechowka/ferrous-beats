use crate::handlers::errors::ServerError;
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Deserialize)]
pub struct DownloadVideoRequest {
    video_url: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadVideoResponse {
    requested_url: String,
    download_video_command_results: CommandExecutionResults,
}

#[instrument(err, skip(app_state))]
pub async fn handle_video_download(
    State(app_state): State<AppState>,
    Json(payload): Json<DownloadVideoRequest>,
) -> Result<(StatusCode, Json<DownloadVideoResponse>), ServerError> {
    debug!("Handling video download");

    todo!()
}
