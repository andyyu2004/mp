pub(crate) mod command;
pub(crate) mod tracklist;

use super::{Key, Region, UI};
use crate::early_return_option;
use crate::{
    cmd::CmdMode, keymap::{self, Handler}
};

pub(crate) trait KeyHandler {}

impl UI {
    fn get_handler(&mut self, key: Key) -> Option<Handler> {
        let client = self.client.lock().unwrap();

        // if first key that enters cmd mode
        if self.uistate.cmd_mode == CmdMode::None {
            if key == Key::Char(':') {
                self.uistate.cmd_mode = CmdMode::Cmd;
                self.uistate.focused_regions.push(Region::StatusBar);
            } else if key == Key::Char('/') {
                self.uistate.cmd_mode = CmdMode::Filter;
                self.uistate.focused_regions.push(Region::StatusBar);
            }
        }

        let region = *self.uistate.focused_regions.last().unwrap();

        if region == Region::StatusBar {
            drop(client);
            self.handle_status_bar_input(key);
            return None;
        }

        let keymap = &client.user_config.keymap;
        let fname = match keymap.get(&(region, key)) {
            Some(name) => name,
            None => {
                warn!("unknown key combination {:?}", key);
                return None;
            }
        };

        match keymap::FMAP.get(fname) {
            Some(&f) => Some(f),
            None => {
                warn!("unknown key combination {:?}", key);
                None
            }
        }
    }

    pub fn handle_keypress(&mut self, key: Key) {
        let handler = early_return_option!(self.get_handler(key));
        handler(self)
    }
}
