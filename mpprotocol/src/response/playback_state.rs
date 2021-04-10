use crate::JoinedTrack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaybackState {
    pub curr_track: Option<JoinedTrack>,
    pub volume: i32,
    pub progress: i64,
    pub is_playing: bool,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            volume: 100,
            curr_track: Option::default(),
            progress: i64::default(),
            is_playing: bool::default(),
        }
    }
}
