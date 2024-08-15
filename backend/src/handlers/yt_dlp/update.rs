use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct YtDlpUpdateRequest {}
#[derive(Debug, Serialize)]
pub struct YtDlpUpdateResponse {
    exists: bool,
    path: String,
}
