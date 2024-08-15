use crate::handlers::errors::ServerError;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::path::Path;
use tokio::fs::metadata;
use tokio::process::Command;
use tracing::{debug, info, instrument};

#[derive(Debug, Serialize)]
pub struct YtDlpStatusResponse {
    path: String,
    executable_version: Option<String>,
    version_execution_results: VersionExecutionResults,
}

#[derive(Debug, Serialize)]
pub struct VersionExecutionResults {
    command_completed_successfully: bool,
    exit_status: Option<i32>,
    stdout: Option<String>,
    stderr: Option<String>,
}

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_status(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<YtDlpStatusResponse>), ServerError> {
    debug!("Handling checking of yt-dlp status");

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

    info!("Running yt-dlp version command");

    let command_output = Command::new(&executable_path)
        .arg("--version")
        .output()
        .await
        .context("Failed to run yt-dlp version command")?;

    let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
    let exit_status = command_output.status.code();
    let command_completed_successfully = exit_status.map_or(false, |code| code == 0);

    info!("yt-dlp version command was executed");

    Ok((
        if command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(YtDlpStatusResponse {
            executable_version: if command_completed_successfully {
                Some(parse_version(&stdout))
            } else {
                None
            },
            path: executable_path.to_string_lossy().to_string(),
            version_execution_results: VersionExecutionResults {
                command_completed_successfully,
                exit_status,
                stdout: Some(stdout),
                stderr: Some(stderr),
            },
        }),
    ))
}

#[instrument]
fn parse_version(version_string: &str) -> String {
    debug!("Parsing yt-dlp version");
    version_string
        .trim()
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}
