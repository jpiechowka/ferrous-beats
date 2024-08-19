use crate::handlers::errors::ServerError;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::path::Path;
use tokio::fs::read_dir;
use tracing::{debug, error, instrument};

#[derive(Debug, Serialize)]
pub struct LibraryListResponse {
    pub library_dir: String,
    pub files: Vec<String>,
}
#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_list_library_files(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<LibraryListResponse>), ServerError> {
    debug!("Handling listing library contents");

    let library_dir = Path::new(&app_state.config.library_settings.dir);
    if !library_dir.exists() {
        error!(
            "Cannot list library, directory does not exist: {}",
            &library_dir.display()
        );
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(LibraryListResponse {
                library_dir: library_dir.to_string_lossy().to_string(),
                files: Vec::new(),
            }),
        ));
    }

    // TODO: For now flat structure is assumed, Change this to support nested directories
    let mut library_files: Vec<String> = Vec::new();
    let mut entries = read_dir(library_dir)
        .await
        .context("Failed to read directory entries")?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .context("Failed to read next entry")?
    {
        let file_name = entry.file_name().to_string_lossy().to_string();
        library_files.push(file_name);
    }

    Ok((
        StatusCode::OK,
        Json(LibraryListResponse {
            library_dir: library_dir.to_string_lossy().to_string(),
            files: library_files,
        }),
    ))
}
