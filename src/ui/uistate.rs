use super::Region;
use tui::widgets::ListState;

pub(crate) struct UIState {
    pub track_list_state: ListState,
    pub focused_region: Region,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            track_list_state: ListState::default(),
            focused_region: Region::TrackList,
        }
    }
}
