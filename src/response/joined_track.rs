use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct JoinedTrack {
    pub track_id: i32,
    pub title: String,
    pub lyrics: String,
    pub comments: String,
    pub genre: String,
    pub track_number: Option<i32>,
    pub path: String,
    pub duration: i32,
    pub bitrate: i32,
    pub samplerate: i32,
    pub channels: i32,
    pub album_id: i32,
    pub album_title: String,
    pub year: Option<i32>,
    pub total_tracks: Option<i32>,
    pub artist_id: i32,
    pub artist_name: String,
}

impl Display for JoinedTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.artist_name, self.title)
    }
}
