use mp_protocol::JoinedTrack;

#[derive(Debug)]
pub(crate) struct ClientState {
    pub tracks: Vec<JoinedTrack>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self { tracks: vec![] }
    }
}
