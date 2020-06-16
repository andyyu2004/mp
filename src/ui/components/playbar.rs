use crate::ui::{render::Render, uistate::UIState};
use crate::util;
use mp_protocol::PlaybackState;
use tui::layout::{Alignment, Constraint, Direction, Layout};
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

        let PlaybackState {
            progress,
            is_playing,
            curr_track,
        } = &state.playback_state;

        let (title_text, subtext, duration) = match curr_track {
            Some(t) => (
                t.title.as_str(),
                format!("\n{} - {}", t.album_title, t.artist_name),
                t.duration as i64 * 1000,
            ),
            None => ("no track playing\n", String::new(), 1),
        };

        // sometimes the calculation can be a bit off due to rounding errors
        let percentage = std::cmp::min(100, progress * 100 / duration);

        let text = [Text::raw(title_text), Text::raw(subtext)];

        let song_info = Paragraph::new(text.iter()).alignment(Alignment::Center);

        f.render_widget(song_info, layout[0]);
        let duration_display = util::format_millis(duration);
        let progress_display = util::format_millis(*progress);
        let remaining_display = util::format_millis(duration - progress);

        let playing_display = if *is_playing { ">>" } else { "||" };
        let song_progress_label = format!(
            "{} {}/{} (-{})",
            playing_display, progress_display, duration_display, remaining_display
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
