use crate::handlers::errors::ServerError;
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Deserialize)]
pub struct DownloadAudioRequest {
    audio_url: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadAudioResponse {
    requested_url: String,
    download_audio_command_results: CommandExecutionResults,
}

#[instrument(err, skip(app_state))]
pub async fn handle_audio_download(
    State(app_state): State<AppState>,
    Json(payload): Json<DownloadAudioRequest>,
) -> Result<(StatusCode, Json<DownloadAudioResponse>), ServerError> {
    debug!("Handling audio download");

    todo!()
}
