use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AcoustIDApiLookupResponse {
    pub status: String,
    pub results: Vec<LookupResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupResult {
    pub id: String,
    pub score: f64,
    pub recordings: Vec<Recording>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recording {
    pub id: String,
}
