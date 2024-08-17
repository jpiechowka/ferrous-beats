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
