use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandExecutionResults {
    pub command_completed_successfully: bool,
    pub exit_status: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
