use crate::ui::{render::Render, uistate::UIState};
use crate::util;
use tui::style::{Color, Modifier, Style};
use tui::{layout::Constraint, widgets::*};

pub(crate) struct Content<'a> {
    uistate: &'a mut UIState,
}

impl<'a> Content<'a> {
    pub fn new(uistate: &'a mut UIState) -> Self {
        Self { uistate }
    }
}

impl Render for Content<'_> {
    fn render<B>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        state: &crate::ClientState,
    ) where
        B: tui::backend::Backend,
    {
        let items = state
            .tracks
            .iter()
            .map(|t| {
                [
                    t.track_number
                        .map(|x| x.to_string())
                        .unwrap_or("".to_owned()),
                    t.title.to_owned(),
                    t.artist_name.to_owned(),
                    t.album_title.to_owned(),
                    t.genre.to_owned(),
                    util::format_millis(t.duration as i64 * 1000),
                ]
            })
            .collect::<Vec<_>>();

        let rows = items.iter().map(|row| Row::Data(row.iter()));

        let headers = ["track", "title", "artist", "album", "genre", "duration"];

        let table = Table::new(headers.iter(), rows)
            .block(Block::default().title("tracks").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .widths(&[
                Constraint::Length(5),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
            ])
            .column_spacing(3)
            .highlight_symbol(">>");

        f.render_stateful_widget(table, rect, &mut self.uistate.track_table_state);
    }
}
