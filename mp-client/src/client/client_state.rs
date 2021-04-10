use mp_protocol::{JoinedTrack, PlaybackState};
use std::collections::VecDeque;

/// holds the state that is transferred from the server to the client
#[derive(Debug, Default)]
pub(crate) struct ClientState {
    pub tracks: Vec<JoinedTrack>,
    pub playback_state: PlaybackState,
    pub history: Vec<JoinedTrack>,
    pub queue: VecDeque<JoinedTrack>,
}
