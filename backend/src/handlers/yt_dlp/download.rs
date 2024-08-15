use crate::handlers::errors::ServerError;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use tracing::{debug, info};

#[derive(Serialize)]
pub struct YtDlpDownloadResponse {
    download_url: String,
    path_on_disk: String,
}

pub async fn yt_dlp_download(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<YtDlpDownloadResponse>), ServerError> {
    debug!("Handling downloading of yt-dlp from GitHub");

    let http_client = Client::new();
    let os = std::env::consts::OS;

    let url = get_download_url(os).await?;

    info!("Downloading yt-dlp from {}", url);

    let resp = http_client
        .get(url)
        .send()
        .await
        .context("Error sending request to download yt-dlp from GitHub")?;

    let content = resp.bytes().await.context("Failed to read response body")?;

    let download_dir = Path::new(&app_state.config.server_settings.dlp_download_dir);
    tokio::fs::create_dir_all(download_dir)
        .await
        .context("Failed to create download directory for yt-dlp")?;

    let file_path = download_dir.join(if os == "windows" {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    });
    let mut file = tokio::fs::File::create_new(file_path.clone())
        .await
        .context("Failed to create new yt-dlp file")?;
    tokio::io::copy(&mut content.as_ref(), &mut file)
        .await
        .context("Failed to write yt-dlp file")?;

    info!("yt-dlp downloaded successfully");

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;

        info!("Setting executable permissions for the file");

        let mut perms = file
            .metadata()
            .await
            .context("Failed to get yt-dlp metadata")?
            .permissions();
        perms.set_mode(0o755);
        file.set_permissions(perms)
            .await
            .context("Failed to set executable permissions for yt-dlp")?;
    }

    Ok((
        StatusCode::OK,
        Json(YtDlpDownloadResponse {
            download_url: url.to_string(),
            path_on_disk: file_path.to_string_lossy().to_string(),
        }),
    ))
}

async fn get_download_url(os: &str) -> Result<&str, anyhow::Error> {
    let url = match os {
        "linux" => "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
        "windows" => "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
        "macos" => "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}
