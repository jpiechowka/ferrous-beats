use crate::handlers::errors::ServerError;
use crate::handlers::shared::functions::files::{decompress_file, search_and_move_binaries};
use crate::handlers::shared::functions::tools::get_chromaprint_download_url_and_output_file_name;
use crate::handlers::shared::model::responses::ToolDownloadResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::path::Path;
use tokio::fs::{create_dir_all, rename, File};
use tokio::io::copy;
use tracing::{debug, info, instrument};

#[instrument(err, skip(app_state))]
pub async fn handle_chromaprint_download(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<ToolDownloadResponse>), ServerError> {
    debug!("Handling downloading of chromaprint");

    let os = std::env::consts::OS;
    let (download_url, output_file_name) =
        get_chromaprint_download_url_and_output_file_name(os).await?;

    info!("Downloading chromaprint from {}", download_url);

    let resp = app_state
        .http_client
        .get(download_url)
        .send()
        .await
        .context("Error sending request to download chromaprint")?;

    let download_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    create_dir_all(download_dir)
        .await
        .context("Failed to create download directory for chromaprint")?;

    let content = resp.bytes().await.context("Failed to read response body")?;

    let download_file_path = download_dir.join(output_file_name);
    let mut file = File::create_new(&download_file_path)
        .await
        .context("Failed to create new chromaprint file")?;
    copy(&mut content.as_ref(), &mut file)
        .await
        .context("Failed to write chromaprint file")?;

    info!("Chromaprint downloaded successfully");
    decompress_file(&download_file_path, &download_dir.to_path_buf())
        .await
        .context("Failed to extract chromaprint archive")?;

    search_and_move_binaries(
        &download_dir.to_path_buf(),
        &download_dir.to_path_buf(),
        &["fpcalc", "fpcalc.exe"],
        1,
    )
    .await
    .context("Failed to move fpcalc binary to the correct location")?;

    Ok((
        StatusCode::OK,
        Json(ToolDownloadResponse {
            download_url: download_url.to_string(),
            path_on_disk: download_file_path.to_string_lossy().to_string(),
        }),
    ))
}
