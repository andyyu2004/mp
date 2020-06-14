pub(crate) mod tracklist;

use super::{Key, UI};
use crate::keymap;

pub(crate) trait Handler {}

impl UI {
    pub fn handle_keypress(&mut self, key: Key) {
        let handler = {
            let client = self.client.lock().unwrap();
            let keymap = &client.user_config.keymap;
            let region = self.uistate.focused_region;
            let fname = match keymap.get(&(region, key)) {
                Some(name) => name,
                None => return warn!("unknown key combination {:?}", key),
            };
            debug!("fname: {}", fname);

            match keymap::FMAP.get(fname) {
                Some(f) => f,
                None => return warn!("unknown function name `{}`", fname),
            }
        };

        handler(self)
    }
}
