use crate::JoinedTrack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaybackState {
    pub curr_track: Option<JoinedTrack>,
    pub duration: i64,
    pub progress: i64,
    pub is_playing: bool,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            curr_track: Option::default(),
            // just to avoid division by 0
            duration: 1,
            progress: i64::default(),
            is_playing: bool::default(),
        }
    }
}
