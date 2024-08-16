use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tracing::{debug, instrument};

#[instrument(err, skip(app_state))]
pub async fn get_yt_dlp_executable_path(app_state: &AppState) -> Result<PathBuf, anyhow::Error> {
    debug!("Getting yt-dlp executable path");

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.dlp_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    });
    let canonical_path = executable_path
        .canonicalize()
        .context("Failed to canonicalize yt-dlp executable path")?;

    Ok(canonical_path)
}

#[instrument(err)]
pub async fn run_command(
    executable_path: &PathBuf,
    args: &[&str],
) -> Result<CommandExecutionResults, anyhow::Error> {
    debug!("Running command");

    let command_output = Command::new(executable_path).args(args).output().await?;

    let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
    let exit_code = command_output.status.code();
    let command_completed_successfully = exit_code.map_or(false, |code| code == 0);

    Ok(CommandExecutionResults {
        command_completed_successfully,
        exit_code,
        stdout: if stdout.is_empty() {
            None
        } else {
            Some(stdout)
        },
        stderr: if stderr.is_empty() {
            None
        } else {
            Some(stderr)
        },
    })
}
