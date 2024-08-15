use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandExecutionResults {
    command_completed_successfully: bool,
    exit_status: Option<i32>,
    stdout: Option<String>,
    stderr: Option<String>,
}
