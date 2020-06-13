mod joined_track;
pub use joined_track::JoinedTrack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    Tracks(Vec<JoinedTrack>),
    Ok,
    Error,
}
