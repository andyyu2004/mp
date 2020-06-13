use crate::ui::{Key, handlers, Region};
use crate::{error::ClientResult, ui::UI, Connection};
use mp_protocol::JoinedTrack;
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct ClientState {
    pub tracks: Vec<JoinedTrack>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self { tracks: vec![] }
    }
}

// can store this in json  later, map to function_name (str) instead.
// then have another map from strings to functions
//
pub(crate) type Handler = for<'r, 'b> fn(&'r mut UI<'b>);
pub(crate) struct KeyMap(HashMap<(Region, Key), Handler>);

impl KeyMap {
    pub fn get<Q>(&self, k: &Q) -> Option<Handler>
    where
        (Region, Key): Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get(k).map(|f| *f)
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        let map = hashmap! {
            (Region::TrackList, Key::Char('j')) => handlers::tracklist::handle_next as Handler,
            (Region::TrackList, Key::Char('k')) => handlers::tracklist::handle_prev as Handler
        };
        Self(map)
    }
}

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
