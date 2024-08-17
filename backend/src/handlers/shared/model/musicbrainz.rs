use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicbrainzAPIRecordingResponse {
    id: String,
    title: String,
    #[serde(rename = "first-release-date")]
    first_release_date: String,
    #[serde(rename = "artist-credit")]
    artist_credit: Vec<ArtistCredit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistCredit {
    name: String,
    artist: Artist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    id: String,
    name: String,
}
