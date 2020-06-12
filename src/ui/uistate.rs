use tui::widgets::ListState;

pub(crate) struct UIState {
    pub track_list_state: ListState,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            track_list_state: ListState::default(),
        }
    }
}
