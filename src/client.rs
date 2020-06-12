use crate::{error::ClientResult, Connection};
use mp_protocol::JoinedTrack;

#[derive(Debug)]
pub struct ClientState {
    pub tracks: Vec<JoinedTrack>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self { tracks: vec![] }
    }
}

pub(crate) struct Client<'a> {
    pub state: ClientState,
    pub connection: &'a mut Connection,
}

impl<'a> Client<'a> {
    pub fn new(connection: &'a mut Connection) -> Self {
        Self {
            connection,
            state: ClientState::default(),
        }
    }

    pub async fn init(&mut self) -> ClientResult<()> {
        self.state.tracks = self.connection.fetch_tracks().await?;
        Ok(())
    }
}
