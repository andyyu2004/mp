mod client_state;
pub(crate) mod keymap;

use crate::network::IOEvent;
use crate::ClientResult;
pub(crate) use client_state::ClientState;
pub(crate) use keymap::KeyMap;
use std::sync::mpsc::Sender;

#[derive(Default)]
pub(crate) struct UserConfig {
    pub keymap: KeyMap,
}

pub(crate) struct Client {
    pub state: ClientState,
    pub user_config: UserConfig,
}

impl Client {
    pub fn new() -> Self {
        Self {
            user_config: UserConfig::default(),
            state: ClientState::default(),
        }
    }

    pub async fn init(&mut self) -> ClientResult<()> {
        //self.state.tracks = self.connection.fetch_tracks().await?;
        Ok(())
    }

    pub async fn play_track(&mut self, track_id: i32) -> ClientResult<()> {
        //self.connection.play_track(track_id).await?;
        Ok(())
    }
}
