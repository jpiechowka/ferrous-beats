use crate::handlers::errors::ServerError;
use crate::handlers::shared::functions::commands::run_command;
use crate::handlers::shared::functions::tools::get_yt_dlp_executable_path;
use crate::handlers::shared::model::media::MediaDownloadResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::path::Path;
use tokio::fs::create_dir_all;
use tracing::{debug, info, instrument};

#[derive(Debug, Deserialize)]
pub struct DownloadAudioRequest {
    audio_url: String,
}

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_audio_download(
    State(app_state): State<AppState>,
    Json(payload): Json<DownloadAudioRequest>,
) -> Result<(StatusCode, Json<MediaDownloadResponse>), ServerError> {
    debug!("Handling audio download");

    let audio_download_dir = Path::new(&app_state.config.audio_download_settings.download_dir);
    create_dir_all(audio_download_dir)
        .await
        .context("Failed to create download directory for audio")?;

    let yt_dlp_executable_path = get_yt_dlp_executable_path(&app_state)
        .await
        .context("Failed to get yt-dlp executable path")?;

    info!("Downloading audio using yt-dlp");

    let output_path = audio_download_dir
        .canonicalize()
        .context("Failed to canonicalize audio output path")?;
    let output_path_str = output_path
        .join("%(title)s.%(ext)s")
        .to_string_lossy()
        .to_string();

    info!("Using output path: {:?}", output_path_str);

    let command_execution_results = run_command(
        &yt_dlp_executable_path,
        &[
            "-x",
            &payload.audio_url,
            "-o",
            &output_path_str,
            "--no-progress",
        ],
    )
    .await
    .context("Failed to download audio using yt-dlp")?;

    info!("yt-dlp audio download command was executed");

    // TODO: Implement moving files to the library directory if successful

    Ok((
        if command_execution_results.command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(MediaDownloadResponse {
            requested_url: payload.audio_url,
            command_execution_results,
        }),
    ))
}
