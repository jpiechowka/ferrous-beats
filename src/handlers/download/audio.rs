use crate::handlers::errors::ServerError;
use crate::handlers::shared::functions::commands::run_command;
use crate::handlers::shared::functions::files::search_and_move_media_file;
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
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

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

    let download_id = Uuid::new_v4();

    let audio_download_dir = Path::new(&app_state.config.audio_download_settings.download_dir);
    create_dir_all(audio_download_dir)
        .await
        .context("Failed to create download directory for audio")?;

    let yt_dlp_executable_path = get_yt_dlp_executable_path(&app_state)
        .await
        .context("Failed to get yt-dlp executable path")?;

    info!(
        "Downloading audio using yt-dlp. Download ID: {}",
        download_id
    );

    let output_path = audio_download_dir
        .canonicalize()
        .context("Failed to canonicalize audio output path")?;
    let output_path_str = output_path
        .join(format!("{}%(title)s.%(ext)s", download_id))
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

    let library_dir = Path::new(&app_state.config.library_settings.dir);
    create_dir_all(library_dir)
        .await
        .context("Failed to create library directory")?;

    if !command_execution_results.command_completed_successfully {
        error!("Failed to download audio using yt-dlp");
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(MediaDownloadResponse {
                download_id: download_id.to_string(),
                library_dir: library_dir.to_string_lossy().to_string(),
                requested_url: payload.audio_url,
                command_execution_results,
            }),
        ));
    }

    info!("Moving downloaded file to the library");
    search_and_move_media_file(
        &audio_download_dir.to_path_buf(),
        &library_dir.to_path_buf(),
        &download_id.to_string(),
        true,
    )
    .await
    .context("Failed to find and move media file")?;

    Ok((
        StatusCode::OK,
        Json(MediaDownloadResponse {
            download_id: download_id.to_string(),
            library_dir: library_dir.to_string_lossy().to_string(),
            requested_url: payload.audio_url,
            command_execution_results,
        }),
    ))
}
