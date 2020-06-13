use super::{key::Key, UI};
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
pub(crate) fn track_list_next(ui: &mut UI) {
    let n = executor::block_on(ui.client.lock()).state.tracks.len();
    if n == 0 { return; }

    let s = &mut ui.uistate.track_list_state;
    let new_index = match s.selected() {
        None => 0,
        Some(i) => (i + 1) % n,
    };

    s.select(Some(new_index));
}

pub(crate) fn track_list_prev(ui: &mut UI) {
    let n = executor::block_on(ui.client.lock()).state.tracks.len();
    if n == 0 { return; }

    let s = &mut ui.uistate.track_list_state;
    let new_index = match s.selected() {
        None => n - 1,
        Some(i) => (i + n - 1) % n,
    };

    s.select(Some(new_index));
}
