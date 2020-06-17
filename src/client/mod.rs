mod client_state;
pub(crate) mod keymap;

pub(crate) use client_state::ClientState;
pub(crate) use keymap::KeyMap;

#[derive(Default)]
pub(crate) struct UserConfig {
    pub keymap: KeyMap,
}

pub(crate) struct Client {
    pub state: ClientState,
    pub user_config: UserConfig,
}

impl Client {
    pub fn new() -> Self {
        Self {
            user_config: UserConfig::default(),
            state: ClientState::default(),
        }
    }
}
