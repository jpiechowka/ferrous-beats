use crate::handlers::errors::ServerError;
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::create_dir_all;
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

    let download_dir = Path::new(&app_state.config.audio_download_settings.download_dir);
    create_dir_all(download_dir)
        .await
        .context("Failed to create download directory for audio")?;

    todo!()
}
