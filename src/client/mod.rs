mod client_state;
mod keymap;

pub(crate) use client_state::ClientState;
pub(crate) use keymap::KeyMap;

use crate::ui::{Key, handlers, Region};
use crate::{error::ClientResult, ui::UI, Connection};
use mp_protocol::JoinedTrack;
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

#[derive(Default)]
pub(crate) struct UserConfig {
    pub keymap: KeyMap,
}

pub(crate) struct Client<'a> {
    pub state: ClientState,
    pub connection: &'a mut Connection,
    pub user_config: UserConfig,
}

impl<'a> Client<'a> {
    pub fn new(connection: &'a mut Connection) -> Self {
        Self {
            connection,
            user_config: UserConfig::default(),
            state: ClientState::default(),
        }
    }

    pub async fn init(&mut self) -> ClientResult<()> {
        self.state.tracks = self.connection.fetch_tracks().await?;
        Ok(())
    }
}
