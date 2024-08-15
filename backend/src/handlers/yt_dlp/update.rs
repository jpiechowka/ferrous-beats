use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct YtDlpUpdateRequest {}
#[derive(Serialize)]
pub struct YtDlpUpdateResponse {
    exists: bool,
    path: String,
}
