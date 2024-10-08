use serde::Serialize;

/// Represents the results of executing a command.
#[derive(Debug, Serialize)]
pub struct CommandExecutionResults {
    /// Indicates whether the command completed successfully.
    pub command_completed_successfully: bool,
    /// The exit code of the command, if available.
    pub exit_code: Option<i32>,
    /// The standard output (stdout) of the command, if any.
    pub stdout: Option<String>,
    /// The standard error (stderr) output of the command, if any.
    pub stderr: Option<String>,
}
