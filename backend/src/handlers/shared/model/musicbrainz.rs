use serde::{Deserialize, Serialize};

// TODO: Check if fields are correctly mapped to MusicBrainz API response fields

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicbrainzAPIRecordingResponse {
    id: String,
    video: bool,
    #[serde(rename = "first-release-date")]
    first_release_date: String,
    length: i32,
    isrcs: Vec<String>,
    disambiguation: String,
    #[serde(rename = "artist-credit")]
    artist_credit: Vec<ArtistCredit>,
    title: String,
    releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistCredit {
    joinphrase: String,
    name: String,
    artist: Artist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    disambiguation: String,
    #[serde(rename = "type")]
    artist_type: String,
    name: String,
    id: String,
    #[serde(rename = "sort-name")]
    sort_name: String,
    #[serde(rename = "type-id")]
    type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    packaging: String,
    date: String,
    #[serde(rename = "packaging-id")]
    packaging_id: String,
    barcode: String,
    #[serde(rename = "text-representation")]
    text_representation: TextRepresentation,
    id: String,
    #[serde(rename = "status-id")]
    status_id: String,
    #[serde(rename = "release-events")]
    release_events: Vec<ReleaseEvent>,
    disambiguation: String,
    title: String,
    status: String,
    quality: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextRepresentation {
    script: String,
    language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseEvent {
    date: String,
    area: Area,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    disambiguation: String,
    #[serde(rename = "iso-3166-1-codes")]
    iso_3166_1_codes: Vec<String>,
    #[serde(rename = "sort-name")]
    sort_name: String,
    id: String,
    #[serde(rename = "type")]
    area_type: Option<String>,
    name: String,
    #[serde(rename = "type-id")]
    type_id: Option<String>,
}
