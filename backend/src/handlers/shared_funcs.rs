use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tracing::{debug, error, info, instrument};

#[instrument(err)]
pub async fn run_command(
    executable_path: &PathBuf,
    args: &[&str],
) -> Result<CommandExecutionResults, anyhow::Error> {
    info!("Running command");

    let command_output = Command::new(executable_path).args(args).output().await?;

    let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
    let exit_code = command_output.status.code();
    let command_completed_successfully = exit_code.map_or(false, |code| code == 0);

    if command_completed_successfully {
        info!("Command completed successfully");
    } else {
        error!("Command failed with exit code: {:?}", exit_code);
    }

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

#[instrument(err)]
async fn decompress_file(path: &PathBuf) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(not(target_os = "windows"))]
#[instrument(err)]
async fn set_executable_permissions(file: &File) -> Result<(), anyhow::Error> {
    use std::os::unix::fs::PermissionsExt;

    info!("Setting executable permissions for the file");

    let mut perms = file
        .metadata()
        .await
        .context("Failed to get file metadata")?
        .permissions();
    perms.set_mode(0o755);
    file.set_permissions(perms)
        .await
        .context("Failed to set executable permissions")?;

    Ok(())
}

#[instrument(err, skip(app_state))]
pub async fn get_yt_dlp_executable_path(app_state: &AppState) -> Result<PathBuf, anyhow::Error> {
    debug!("Getting yt-dlp executable path");

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    });
    let canonical_path = executable_path
        .canonicalize()
        .context("Failed to canonicalize yt-dlp executable path. One of the reasons can be that the executable does not exist")?;

    Ok(canonical_path)
}

#[instrument(err, skip(app_state))]
pub async fn get_ffmpeg_executable_path(app_state: &AppState) -> Result<PathBuf, anyhow::Error> {
    debug!("Getting ffmpeg executable path");

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    });
    let canonical_path = executable_path
        .canonicalize()
        .context("Failed to canonicalize ffmpeg executable path. One of the reasons can be that the executable does not exist")?;

    Ok(canonical_path)
}
