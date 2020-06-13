pub(crate) mod tracklist;

use super::{Key, UI};

// can't get types to work out when the handlers are defined as methods on UI
impl UI<'_> {
    pub async fn handle_keypress(&mut self, key: Key) {
        let handler = {
            let client = self.client.lock().await;
            let keymap = &client.user_config.keymap;
            let region = self.uistate.focused_region;
            match keymap.get(&(region, key)) {
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
