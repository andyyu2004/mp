use crate::ui::{Key, Region, UI};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

const SELECT_PREV: &str = "select_prev";
const SELECT_NEXT: &str = "select_next";
const PLAY_TRACK: &str = "play_track";
const TOGGLE_PLAY: &str = "toggle_play";
const APPEND_TO_QUEUE: &str = "queue_append";
const PLAY_NEXT: &str = "play_next";
const PLAY_PREV: &str = "play_prev";
const SEEK_BACKWARD: &str = "seek_backward";
const SEEK_FORWARD: &str = "seek_forward";
const SET_NEXT_TRACK: &str = "set_next";
const SHUFFLE_ALL: &str = "shuffle_all";
const VOLUME_DOWN: &str = "volume_down";
const VOLUME_UP: &str = "volume_up";

lazy_static! {
    /// map from command names to the handler
    pub(crate) static ref FMAP: HashMap<&'static str, Handler> = hashmap! {
        SELECT_PREV => UI::handle_select_prev as Handler,
        SELECT_NEXT=> UI::handle_select_next,
        PLAY_TRACK => UI::handle_play_track,
        TOGGLE_PLAY => UI::handle_toggle_play,
        APPEND_TO_QUEUE => UI::handle_queue_append,
        PLAY_PREV => UI::handle_play_prev,
        PLAY_NEXT => UI::handle_play_next,
        SHUFFLE_ALL => UI::handle_shuffle_all,
        SEEK_BACKWARD => UI::handle_seek_backward,
        SEEK_FORWARD => UI::handle_seek_forward,
        SET_NEXT_TRACK => UI::handle_set_next,
        VOLUME_DOWN => UI::handle_volume_down,
        VOLUME_UP => UI::handle_volume_up,
    };
}

// pub(crate) type Handler = for<'r> fn(&'r mut UI);
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
        Self(hashmap! {
            (Region::TrackList, Key::Char('j')) => SELECT_NEXT,
            (Region::TrackList, Key::Char('k')) => SELECT_PREV,
            (Region::TrackList, Key::Ctrl('d')) => TOGGLE_PLAY,
            (Region::TrackList, Key::Ctrl('q')) => APPEND_TO_QUEUE,
            (Region::TrackList, Key::Ctrl('f')) => PLAY_NEXT,
            (Region::TrackList, Key::Ctrl('s')) => PLAY_PREV,
            (Region::TrackList, Key::Ctrl('h')) => SEEK_BACKWARD,
            (Region::TrackList, Key::Ctrl('l')) => SEEK_FORWARD,
            (Region::TrackList, Key::Ctrl('n')) => SET_NEXT_TRACK,
            (Region::TrackList, Key::Alt('s')) => SHUFFLE_ALL,
            (Region::TrackList, Key::Alt('j')) => VOLUME_DOWN,
            (Region::TrackList, Key::Alt('k')) => VOLUME_UP,
            (Region::TrackList, Key::Enter) => PLAY_TRACK,
        })
    }
}
