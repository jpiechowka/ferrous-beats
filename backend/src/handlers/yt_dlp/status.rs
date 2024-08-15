use crate::handlers::errors::ServerError;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::path::Path;
use tokio::fs::metadata;
use tokio::process::Command;
use tracing::{debug, error, info, instrument};

#[derive(Debug, Serialize)]
pub struct YtDlpStatusResponse {
    executable_exists: bool,
    path: String,
    executable_version: Option<String>,
    version_execution_results: VersionExecutionResults,
}

#[derive(Debug, Serialize)]
pub struct VersionExecutionResults {
    was_version_command_run: bool,
    command_completed_successfully: bool,
    command_error: Option<String>,
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

    let executable_exists = metadata(&executable_path).await.is_ok();
    if !executable_exists {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(YtDlpStatusResponse {
                executable_exists: false,
                executable_version: None,
                path: executable_path.to_string_lossy().to_string(),
                version_execution_results: VersionExecutionResults {
                    was_version_command_run: false,
                    command_completed_successfully: false,
                    command_error: None,
                    exit_status: None,
                    stdout: None,
                    stderr: None,
                },
            }),
        ));
    }

    info!("Running yt-dlp version command");

    match Command::new(&executable_path)
        .arg("--versionz")
        .output()
        .await
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_status = output.status.code();
            let command_completed_successfully = exit_status.map_or(false, |code| code == 0);

            info!("yt-dlp version command was executed");

            Ok((
                if command_completed_successfully {
                    StatusCode::OK
                } else {
                    StatusCode::BAD_REQUEST
                },
                Json(YtDlpStatusResponse {
                    executable_exists: true,
                    executable_version: if command_completed_successfully {
                        Some(parse_version(&stdout))
                    } else {
                        None
                    },
                    path: executable_path.to_string_lossy().to_string(),
                    version_execution_results: VersionExecutionResults {
                        was_version_command_run: true,
                        command_completed_successfully,
                        command_error: None,
                        exit_status,
                        stdout: Some(stdout),
                        stderr: Some(stderr),
                    },
                }),
            ))
        }
        Err(err) => {
            error!("Error running yt-dlp version command: {}", err);

            Ok((
                StatusCode::BAD_REQUEST,
                Json(YtDlpStatusResponse {
                    executable_exists: true,
                    executable_version: None,
                    path: executable_path.to_string_lossy().to_string(),
                    version_execution_results: VersionExecutionResults {
                        was_version_command_run: true,
                        command_completed_successfully: false,
                        command_error: Some(err.to_string()),
                        exit_status: None,
                        stdout: None,
                        stderr: None,
                    },
                }),
            ))
        }
    }
}

#[instrument]
fn parse_version(version_string: &str) -> String {
    debug!("Parsing yt-dlp version string: {}", version_string);
    version_string
        .trim()
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}
