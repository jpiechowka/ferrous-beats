use crate::handlers::errors::ServerError;
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::Path;
use strum_macros::Display;
use tokio::fs::metadata;
use tokio::process::Command;
use tracing::{debug, info, instrument};

#[derive(Debug, Deserialize)]
pub struct YtDlpUpdateRequest {
    update_channel: YtDlpUpdateChannels,
}

#[derive(Debug, Display, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YtDlpUpdateChannels {
    Stable,
    Master,
    Nightly,
}

#[derive(Debug, Serialize)]
pub struct YtDlpUpdateResponse {
    path: String,
    update_channel: String,
    update_execution_results: CommandExecutionResults,
}

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_update(
    State(app_state): State<AppState>,
    Json(payload): Json<YtDlpUpdateRequest>,
) -> Result<(StatusCode, Json<YtDlpUpdateResponse>), ServerError> {
    debug!("Handling updating of yt-dlp");

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

    info!("Running yt-dlp update command");

    let update_arg = format!(
        "{}@latest",
        payload.update_channel.to_string().to_lowercase()
    );
    info!("Updating yt-dlp to {}", &update_arg);

    let command_output = Command::new(&executable_path)
        .arg("--update-to")
        .arg(&update_arg)
        .output()
        .await
        .context("Failed to run yt-dlp update command")?;

    let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
    let exit_status = command_output.status.code();
    let command_completed_successfully = exit_status.map_or(false, |code| code == 0);

    info!("yt-dlp update command was executed");

    Ok((
        if command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(YtDlpUpdateResponse {
            path: executable_path.to_string_lossy().to_string(),
            update_channel: update_arg,
            update_execution_results: UpdateExecutionResults {
                command_completed_successfully,
                exit_status,
                stdout: Some(stdout),
                stderr: Some(stderr),
            },
        }),
    ))
}
