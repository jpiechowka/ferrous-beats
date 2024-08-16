use crate::handlers::errors::ServerError;
use crate::handlers::shared_funcs::{get_yt_dlp_executable_path, run_command};
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use tracing::{debug, info, instrument};

#[derive(Debug, Serialize)]
pub struct YtDlpStatusResponse {
    path: String,
    executable_version: Option<String>,
    version_execution_results: CommandExecutionResults,
}

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_status(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<YtDlpStatusResponse>), ServerError> {
    debug!("Handling checking of yt-dlp status");

    let yt_dlp_executable_path = get_yt_dlp_executable_path(&app_state)
        .await
        .context("Failed to get yt-dlp executable path")?;

    info!("Running yt-dlp version command");
    let command_execution_result = run_command(&yt_dlp_executable_path, &["--version"])
        .await
        .context("Failed to run yt-dlp version command")?;

    info!("yt-dlp version command was executed");

    Ok((
        if command_execution_result.command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(YtDlpStatusResponse {
            executable_version: if command_execution_result.command_completed_successfully {
                Some(parse_yt_dlp_version(&command_execution_result.stdout))
            } else {
                None
            },
            path: yt_dlp_executable_path.to_string_lossy().to_string(),
            version_execution_results: command_execution_result,
        }),
    ))
}

#[instrument]
fn parse_yt_dlp_version(version_string: &Option<String>) -> String {
    debug!("Parsing yt-dlp version");
    match version_string {
        None => String::from("could not parse version"),
        Some(ver) => ver.trim().chars().filter(|c| !c.is_control()).collect(),
    }
}
