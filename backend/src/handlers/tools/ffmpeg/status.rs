use crate::handlers::errors::ServerError;
use crate::handlers::shared_funcs::{get_ffmpeg_executable_path, run_command};
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use tracing::{debug, instrument};

#[derive(Debug, Serialize)]
pub struct FfmpegStatusResponse {
    path: String,
    executable_version: Option<String>,
    version_execution_results: CommandExecutionResults,
}

#[instrument(err, skip(app_state))]
pub async fn handle_ffmpeg_status(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<FfmpegStatusResponse>), ServerError> {
    debug!("Handling checking of ffmpeg status");

    let ffmpeg_executable_path = get_ffmpeg_executable_path(&app_state)
        .await
        .context("Failed to get ffmpeg executable path")?;

    let command_execution_result = run_command(&ffmpeg_executable_path, &["-version"])
        .await
        .context("Failed to run ffmpeg version command")?;

    Ok((
        if command_execution_result.command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(FfmpegStatusResponse {
            executable_version: if command_execution_result.command_completed_successfully {
                Some(parse_ffmpeg_version(&command_execution_result.stdout))
            } else {
                None
            },
            path: ffmpeg_executable_path.to_string_lossy().to_string(),
            version_execution_results: command_execution_result,
        }),
    ))
}

#[instrument]
fn parse_ffmpeg_version(version_string: &Option<String>) -> String {
    debug!("Parsing ffmpeg version");
    match version_string {
        None => String::from("could not parse version"),
        Some(ver) => ver
            .trim()
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>()
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown version")
            .to_string(),
    }
}
