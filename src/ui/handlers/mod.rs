mod tracklist;

use super::{Key, UI, Region};
use futures::executor;

impl UI<'_> {
    pub async fn handle_keypress(&mut self, key: Key) {
        let handler = {
            let client = self.client.lock().await;
            let keymap = &client.user_config.keymap;
            match keymap.get(&key) {
                Some(f) => f,
                None => {
                    info!("unknown key combination {:?}", key);
                    return;
                }
            }
        };

        handler(self)
    }
}

// can't get types to work out when these are defined as methods on UI
pub(crate) fn handle_j_pressed(ui: &mut UI) {
    match ui.uistate.focused_region {
        Region::TrackList => tracklist::handle_next(ui)
    }
}

pub(crate) fn handle_k_pressed(ui: &mut UI) {
    match ui.uistate.focused_region {
        Region::TrackList => tracklist::handle_prev(ui),
    }
}

