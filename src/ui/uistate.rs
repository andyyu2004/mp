use super::Region;
use tui::widgets::TableState;

pub(crate) struct UIState {
    pub track_table_state: TableState,
    pub focused_region: Region,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            track_table_state: TableState::default(),
            focused_region: Region::TrackList,
        }
    }
}
