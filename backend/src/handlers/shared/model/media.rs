use crate::handlers::shared::model::commands::CommandExecutionResults;
use serde::Serialize;

/// Represents the response for a media download operation.
#[derive(Debug, Serialize)]
pub struct MediaDownloadResponse {
    /// ID of the download operation. Will be used to move files from the download directory to the library directory.
    pub download_id: String,
    /// The URL from which the media was downloaded.
    pub requested_url: String,
    /// The local file system path to the media library the file will be moved to after successful download.
    pub library_dir: String,
    /// The results of executing the command to download the media.
    pub command_execution_results: CommandExecutionResults,
}
