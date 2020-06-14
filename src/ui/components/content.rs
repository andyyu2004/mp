use crate::ui::{render::Render, uistate::UIState};
use tui::style::{Color, Modifier, Style};
use tui::widgets::*;

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
        let strings = state.tracks.iter().map(|t| &t.title);
        let items = strings.map(Text::raw);
        let list = List::new(items)
            .block(Block::default().title("tracks").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        f.render_stateful_widget(list, rect, &mut self.uistate.track_list_state);
    }
}
