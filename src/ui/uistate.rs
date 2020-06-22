use super::Region;
use crate::cmd::{CmdMode, TrackFilter};
use tui::widgets::TableState;

pub(crate) struct UIState {
    pub track_table_state: TableState,
    pub filtered_tracklist_len: usize,
    pub focused_regions: Vec<Region>,
    /// the current command being entered
    pub cmd: String,
    pub cmd_mode: CmdMode,
    pub filter: TrackFilter,
    pub should_quit: bool,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            track_table_state: TableState::default(),
            focused_regions: vec![Region::TrackList],
            filtered_tracklist_len: 0,
            cmd_mode: Default::default(),
            cmd: Default::default(),
            filter: Default::default(),
            should_quit: Default::default(),
        }
    }
}
