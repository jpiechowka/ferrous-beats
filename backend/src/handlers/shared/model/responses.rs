use crate::handlers::shared::model::commands::CommandExecutionResults;
use serde::Serialize;

/// Represents the response for a media download operation.
#[derive(Debug, Serialize)]
pub struct MediaDownloadResponse {
    /// The URL from which the media was downloaded.
    pub requested_url: String,
    /// The results of executing the command to download the media.
    pub command_execution_results: CommandExecutionResults,
}

/// Represents the response for a successful tool download operation.
#[derive(Debug, Serialize)]
pub struct ToolDownloadResponse {
    /// The URL from which the tool was downloaded.
    pub download_url: String,
    /// The local file system directory where the downloaded tool is stored.
    pub tools_dir_path: String,
}

/// Represents the response for a tool status check operation. This usually will use the version command of the tool to determine its status.
#[derive(Debug, Serialize)]
pub struct ToolStatusResponse {
    /// The local file system path to the tool executable.
    pub path: String,
    /// The version of the executable, if available.
    pub executable_version: Option<String>,
    /// The results of executing the version command on the tool.
    pub command_execution_results: CommandExecutionResults,
}
