use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct JoinedTrack {
    track_id: i32,
    title: String,
    lyrics: String,
    comments: String,
    genre: String,
    track_number: Option<i32>,
    path: String,
    duration: i32,
    bitrate: i32,
    samplerate: i32,
    channels: i32,
    album_id: i32,
    album_title: String,
    year: Option<i32>,
    total_tracks: Option<i32>,
    artist_id: i32,
    artist_name: String,
}
