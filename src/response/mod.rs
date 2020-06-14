mod joined_track;
mod playback_state;

pub use joined_track::JoinedTrack;
pub use playback_state::PlaybackState;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    Tracks(Vec<JoinedTrack>),
    Q(Vec<JoinedTrack>, VecDeque<JoinedTrack>),
    PlaybackState(PlaybackState),
    Ok,
    Error,
}
