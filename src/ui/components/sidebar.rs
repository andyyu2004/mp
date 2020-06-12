use crate::ui::render::Render;
use tui::widgets::*;

pub(crate) struct Sidebar;

impl Render for Sidebar {
    fn render<B>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        state: &crate::ClientState,
    ) where
        B: tui::backend::Backend,
    {
        f.render_widget(
            Block::default().title("sidebar").borders(Borders::ALL),
            rect,
        );
    }
}
