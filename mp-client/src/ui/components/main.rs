use super::{Content, Sidebar};
use crate::ui::{Render, UIState};
use tui::layout::{Constraint, Direction, Layout};

pub(crate) struct Main<'a> {
    uistate: &'a mut UIState,
}

impl<'a> Main<'a> {
    pub fn new(uistate: &'a mut UIState) -> Self {
        Self { uistate }
    }
}

impl Render for Main<'_> {
    fn render<B>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        state: &crate::ClientState,
    ) where
        B: tui::backend::Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(30), Constraint::Min(1)])
            .split(rect);

        Sidebar.render(f, layout[0], state);
        Content::new(self.uistate).render(f, layout[1], state);
    }
}
