use crate::ui::{render::Render, uistate::UIState};
use crate::util;
use mp_protocol::PlaybackState;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::*;

pub(crate) struct Playbar<'a> {
    uistate: &'a mut UIState,
}

impl<'a> Playbar<'a> {
    pub fn new(uistate: &'a mut UIState) -> Self {
        Self { uistate }
    }
}

impl Render for Playbar<'_> {
    fn render<B>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        state: &crate::ClientState,
    ) where
        B: tui::backend::Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);

        let song_progress_label = "song_progress";
        let PlaybackState {
            progress,
            duration,
            is_playing,
            curr_track,
        } = &state.playback_state;
        let percentage = state.playback_state.progress * 100 / state.playback_state.duration;

        let duration_display = util::format_millis(*duration);
        let progress_display = util::format_millis(*progress);
        let remaining_display = util::format_millis(duration - progress);

        let song_progress_label = format!(
            "{}/{} (-{})",
            progress_display, duration_display, remaining_display
        );

        let song_progress = Gauge::default()
            .block(Block::default().borders(Borders::ALL))
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .percent(percentage as u16)
            .label(&song_progress_label);
        f.render_widget(song_progress, layout[1]);
    }
}
