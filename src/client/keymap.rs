use crate::ui::{Key, Region, UI};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

lazy_static! {
    /// map from strings to the function
    pub(crate) static ref FMAP: HashMap<&'static str, Handler> = hashmap! {
        "select_prev" => UI::handle_select_prev as Handler,
        "select_next" => UI::handle_select_next,
        "play_track" => UI::handle_play_track,
        "toggle_play" => UI::handle_toggle_play,
        "queue_append" => UI::handle_queue_append,
        "play_prev" => UI::handle_play_prev,
        "play_next" => UI::handle_play_next,
        "shuffle_all" => UI::handle_shuffle_all,
        "enter_command" => UI::handle_enter_command,
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
            (Region::TrackList, Key::Ctrl('q')) => "queue_append",
            (Region::TrackList, Key::Ctrl('f')) => "play_next",
            (Region::TrackList, Key::Ctrl('s')) => "play_prev",
            (Region::TrackList, Key::Alt('s')) => "shuffle_all",
            (Region::TrackList, Key::Ctrl(':')) => "enter_command",
        };
        Self(map)
    }
}
