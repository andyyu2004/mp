use crate::ui::{Key, Region, UI};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

lazy_static! {
    /// map from strings to the function
    pub(crate) static ref FMAP: HashMap<&'static str, Handler> = hashmap! {
        "select_prev" => UI::handle_prev_track as Handler,
        "select_next" => UI::handle_next_track,
        "play_track" => UI::handle_play_track,
        "toggle_play" => UI::handle_toggle_play,
        "queue_append" => UI::handle_queue_append,
    };
}

pub(crate) type Handler = for<'r> fn(&'r mut UI);
pub(crate) struct KeyMap(HashMap<(Region, Key), &'static str>);

impl KeyMap {
    pub fn get<Q>(&self, k: &Q) -> Option<&str>
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
            (Region::TrackList, Key::Char('j')) => "select_next",
            (Region::TrackList, Key::Char('k')) => "select_prev",
            (Region::TrackList, Key::Enter) => "play_track",
            (Region::TrackList, Key::Ctrl('d')) => "toggle_play",
            (Region::TrackList, Key::Ctrl('a')) => "queue_append",
        };
        Self(map)
    }
}
