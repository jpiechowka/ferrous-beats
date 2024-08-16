use crate::handlers::errors::ServerError;
use crate::handlers::shared_model::ToolDownloadResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use reqwest::Client;
use std::path::Path;
use tokio::fs::{create_dir_all, File};
use tokio::io::copy;
use tracing::{debug, info, instrument};

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_download(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<ToolDownloadResponse>), ServerError> {
    debug!("Handling downloading of yt-dlp from GitHub");

    let http_client = Client::new();
    let os = std::env::consts::OS;
    let (download_url, output_file_name) = get_yt_dlp_download_url_and_output_file_name(os).await?;

    info!("Downloading yt-dlp from {}", download_url);

    let resp = http_client
        .get(download_url)
        .send()
        .await
        .context("Error sending request to download yt-dlp from GitHub")?;

    let content = resp.bytes().await.context("Failed to read response body")?;

    let download_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    create_dir_all(download_dir)
        .await
        .context("Failed to create download directory for yt-dlp")?;

    let download_file_path = download_dir.join(output_file_name);
    let mut file = File::create_new(&download_file_path)
        .await
        .context("Failed to create new yt-dlp file")?;
    copy(&mut content.as_ref(), &mut file)
        .await
        .context("Failed to write yt-dlp file")?;

    info!("yt-dlp downloaded successfully");

    #[cfg(not(target_os = "windows"))]
    set_executable_permissions(&file)
        .await
        .context("Failed to set yt-dlp executable permissions")?;

    Ok((
        StatusCode::OK,
        Json(ToolDownloadResponse {
            download_url: download_url.to_string(),
            path_on_disk: download_file_path.to_string_lossy().to_string(),
        }),
    ))
}

#[instrument(err)]
async fn get_yt_dlp_download_url_and_output_file_name(
    os: &str,
) -> Result<(&str, &str), anyhow::Error> {
    // https://github.com/yt-dlp/yt-dlp/releases
    let url = match os {
        "linux" => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
            "yt-dlp",
        ),
        "windows" => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
            "yt-dlp.exe",
        ),
        "macos" => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
            "yt-dlp",
        ),
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}
