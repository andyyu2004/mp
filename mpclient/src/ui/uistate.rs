use crate::cmd::{CmdMode, TrackFilter};
use crate::ui::Render;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, TableState};
use tui::Frame;

use super::components::{CommandBar, Main, Playbar, StatusBar};
use super::Region;

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

impl UIState {
    fn get_layout(&self) -> Layout {
        let constraints = [
            // top bar
            Constraint::Length(3),
            // main
            Constraint::Min(1),
            // play bar
            Constraint::Length(6),
            // status bar
            // Constraint::Length(3),
            // command bar
            Constraint::Length(3),
        ];

        Layout::default().direction(Direction::Vertical).constraints(constraints)
    }
}

impl Render for UIState {
    fn render<B>(&mut self, f: &mut Frame<B>, rect: Rect, state: &crate::ClientState)
    where
        B: Backend,
    {
        let layout = self.get_layout().split(rect);
        let [top_bar_rect, main_rect, playbar_rect, commandbar_rect] = match layout.as_slice() {
            &[a, b, c, d] => [a, b, c, d],
            _ => panic!(),
        };
        let block = Block::default().title("top").borders(Borders::ALL);
        f.render_widget(block, top_bar_rect);

        Main::new(self).render(f, main_rect, state);
        Playbar::new(self).render(f, playbar_rect, state);
        // StatusBar::new(self).render(f, statusbar_rect, state);
        CommandBar::new(self).render(f, commandbar_rect, state);
    }
}
