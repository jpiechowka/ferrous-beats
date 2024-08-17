use crate::handlers::shared::model::commands::CommandExecutionResults;
use std::path::PathBuf;
use tokio::process::Command;
use tracing::{error, info, instrument};

#[instrument(err, ret(level = "debug"))]
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
