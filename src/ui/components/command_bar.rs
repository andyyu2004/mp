use crate::ui::{Render, UIState};
use tui::{layout::Alignment, widgets::*};

pub(crate) struct CommandBar<'a> {
    uistate: &'a mut UIState,
}

impl<'a> CommandBar<'a> {
    pub fn new(uistate: &'a mut UIState) -> Self {
        Self { uistate }
    }
}

impl Render for CommandBar<'_> {
    fn render<B>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        _state: &crate::client::ClientState,
    ) where
        B: tui::backend::Backend,
    {
        let cmd = [Text::raw(&self.uistate.cmd)];
        let paragraph = Paragraph::new(cmd.iter())
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL))
            .wrap(true);
        f.render_widget(paragraph, rect);
    }
}
