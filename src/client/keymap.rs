use std::hash::Hash;
use std::borrow::Borrow;
use crate::ui::{handlers, Key,Region, UI};
use std::collections::HashMap;

// can store this in json  later, map to function_name (str) instead.
// then have another map from strings to functions
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

