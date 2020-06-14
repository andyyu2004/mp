use mp_protocol::{JoinedTrack, PlaybackState};

/// holds the state that is transferred from the server to the client
#[derive(Debug)]
pub(crate) struct ClientState {
    pub tracks: Vec<JoinedTrack>,
    pub playback_state: PlaybackState,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            tracks: vec![],
            playback_state: PlaybackState::default(),
        }
    }
}
