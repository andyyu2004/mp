use crate::ui::{Render, UIState};
use tui::widgets::*;

pub(crate) struct StatusBar<'a> {
    uistate: &'a mut UIState,
}

impl<'a> StatusBar<'a> {
    pub fn new(uistate: &'a mut UIState) -> Self {
        Self { uistate }
    }
}

impl Render for StatusBar<'_> {
    fn render<B>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        state: &crate::client::ClientState,
    ) where
        B: tui::backend::Backend,
    {
        let block = Block::default().title("").borders(Borders::ALL);
        f.render_widget(block, rect);
    }
}
