use crate::handlers::errors::ServerError;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use tokio::fs::{create_dir_all, File};
use tokio::io::copy;
use tracing::{debug, info, instrument};

#[derive(Debug, Serialize)]
pub struct FfmpegDownloadResponse {
    download_url: String,
    path_on_disk: String,
}

#[instrument(err, skip(app_state))]
pub async fn handle_ffmpeg_download(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<FfmpegDownloadResponse>), ServerError> {
    debug!("Handling downloading of ffmpeg");

    let http_client = Client::new();
    let os = std::env::consts::OS;
    let (download_url, output_file_name) = get_ffmpeg_download_url_and_output_file_name(os).await?;

    info!("Downloading ffmpeg from {}", download_url);

    let resp = http_client
        .get(download_url)
        .send()
        .await
        .context("Error sending request to download ffmpeg")?;

    let content = resp.bytes().await.context("Failed to read response body")?;

    let download_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    create_dir_all(download_dir)
        .await
        .context("Failed to create download directory for yt-dlp")?;

    let download_file_path = download_dir.join(output_file_name);
    let mut file = File::create_new(&download_file_path)
        .await
        .context("Failed to create new ffmpeg file")?;
    copy(&mut content.as_ref(), &mut file)
        .await
        .context("Failed to write ffmpeg file")?;

    info!("ffmpeg downloaded successfully");

    // TODO extract ffmpeg
    // TODO: set permissions

    Ok((
        StatusCode::OK,
        Json(FfmpegDownloadResponse {
            download_url: download_url.to_string(),
            path_on_disk: download_file_path.to_string_lossy().to_string(),
        }),
    ))
}
#[instrument(err)]
async fn get_ffmpeg_download_url_and_output_file_name(
    os: &str,
) -> Result<(&str, &str), anyhow::Error> {
    // https://ffmpeg.org/download.html
    let url = match os {
        "linux" => (
            "https://johnvansickle.com/ffmpeg/builds/ffmpeg-git-amd64-static.tar.xz",
            "ffmpeg.tar.xz",
        ),
        "windows" => (
            "https://www.gyan.dev/ffmpeg/builds/ffmpeg-git-full.7z",
            "ffmpeg.7z",
        ),
        "macos" => ("https://evermeet.cx/ffmpeg/ffmpeg-7.0.2.7z", "ffmpeg.7z"),
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}
