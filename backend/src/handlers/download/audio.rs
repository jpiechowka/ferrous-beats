use crate::handlers::errors::ServerError;
use crate::handlers::shared_model::CommandExecutionResults;
use crate::handlers::yt_dlp::status::YtDlpStatusResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::{create_dir_all, metadata};
use tokio::process::Command;
use tracing::{debug, info, instrument};
use tracing_subscriber::fmt::format;

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

    let audio_download_dir = Path::new(&app_state.config.audio_download_settings.download_dir);
    create_dir_all(audio_download_dir)
        .await
        .context("Failed to create download directory for audio")?;

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.dlp_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    });

    // Check if executable exists first
    metadata(&executable_path)
        .await
        .context("Failed to get metadata of yt-dlp executable")?;

    info!("Downloading audio using yt-dlp");

    let output_path = audio_download_dir
        .canonicalize()
        .context("Failed to canonicalize audio output path")?;
    let output_path_str = output_path
        .join("%(title)s.%(ext)s")
        .to_string_lossy()
        .to_string();

    info!("Using output path: {}", output_path_str);

    // TODO: for now it requires ffmpeg to be installed
    let command_output = Command::new(&executable_path)
        .arg("-x")
        .arg(&payload.audio_url)
        .arg("-o")
        .arg(output_path_str)
        .arg("--no-progress")
        .output()
        .await
        .context("Failed to download audio using yt-dlp")?;

    let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
    let exit_status = command_output.status.code();
    let command_completed_successfully = exit_status.map_or(false, |code| code == 0);

    info!("yt-dlp audio download command was executed");

    Ok((
        if command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(DownloadAudioResponse {
            requested_url: payload.audio_url,
            download_audio_command_results: CommandExecutionResults {
                command_completed_successfully,
                exit_status,
                stdout: Some(stdout),
                stderr: Some(stderr),
            },
        }),
    ))
}
