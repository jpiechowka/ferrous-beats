use crate::handlers::errors::ServerError;
use crate::handlers::shared::functions::files::{decompress_file, search_and_move_binaries};
use crate::handlers::shared::functions::tools::get_ffmpeg_download_url_and_output_file_name;
use crate::handlers::shared::model::responses::ToolDownloadResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::path::Path;
use tokio::fs::{create_dir_all, File};
use tokio::io::copy;
use tracing::{debug, info, instrument};

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_ffmpeg_download(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<ToolDownloadResponse>), ServerError> {
    debug!("Handling downloading of ffmpeg");

    let os = std::env::consts::OS;
    let (download_url, output_file_name) = get_ffmpeg_download_url_and_output_file_name(os).await?;

    info!("Downloading ffmpeg from {}", download_url);

    let resp = app_state
        .http_client
        .get(download_url)
        .send()
        .await
        .context("Error sending request to download ffmpeg")?;

    let content = resp.bytes().await.context("Failed to read response body")?;

    let download_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    create_dir_all(download_dir)
        .await
        .context("Failed to create download directory for ffmpeg")?;

    let download_file_path = download_dir.join(output_file_name);
    let mut file = File::create_new(&download_file_path)
        .await
        .context("Failed to create new ffmpeg file")?;
    copy(&mut content.as_ref(), &mut file)
        .await
        .context("Failed to write ffmpeg file")?;

    info!("ffmpeg downloaded successfully");
    decompress_file(&download_file_path, &download_dir.to_path_buf())
        .await
        .context("Failed to extract ffmpeg archive")?;

    search_and_move_binaries(
        &download_dir.to_path_buf(),
        &download_dir.to_path_buf(),
        &[
            "ffmpeg",
            "ffplay",
            "ffprobe",
            "ffmpeg.exe",
            "ffplay.exe",
            "ffprobe.exe",
        ],
        3,
    )
    .await
    .context("Failed to move ffmpeg binaries to the correct location")?;

    // TODO: Remove the downloaded file after successful move and extraction?
    // TODO: Is it required to change file permissions after extraction?

    Ok((
        StatusCode::OK,
        Json(ToolDownloadResponse {
            download_url: download_url.to_string(),
            path_on_disk: download_file_path.to_string_lossy().to_string(),
        }),
    ))
}
