use crate::handlers::errors::ServerError;
use crate::handlers::shared::functions::commands::run_command;
use crate::handlers::shared::functions::tools::get_ffmpeg_executable_path;
use crate::handlers::shared::model::responses::ToolStatusResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tracing::{debug, instrument};

#[instrument(err, skip(app_state))]
pub async fn handle_ffmpeg_status(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<ToolStatusResponse>), ServerError> {
    debug!("Handling checking of ffmpeg status");

    let ffmpeg_executable_path = get_ffmpeg_executable_path(&app_state)
        .await
        .context("Failed to get ffmpeg executable path")?;

    let command_execution_results = run_command(&ffmpeg_executable_path, &["-version"])
        .await
        .context("Failed to run ffmpeg version command")?;

    Ok((
        if command_execution_results.command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(ToolStatusResponse {
            executable_version: if command_execution_results.command_completed_successfully {
                Some(parse_ffmpeg_version(&command_execution_results.stdout))
            } else {
                None
            },
            path: ffmpeg_executable_path.to_string_lossy().to_string(),
            command_execution_results,
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
