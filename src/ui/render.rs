use super::components::*;
use super::uistate::UIState;
use tui::backend::Backend;
use tui::layout::*;
use tui::widgets::*;
use tui::Frame;

pub(crate) trait Render {
    fn render<B>(&mut self, f: &mut Frame<B>, rect: Rect, state: &crate::ClientState)
    where
        B: Backend;
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
            Constraint::Length(3),
        ];

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
    }
}

impl Render for UIState {
    fn render<B>(&mut self, f: &mut Frame<B>, rect: Rect, state: &crate::ClientState)
    where
        B: Backend,
    {
        let layout = self.get_layout().split(rect);
        let (top_bar_rect, main_rect, playbar_rect, statusbar_rect) =
            (layout[0], layout[1], layout[2], layout[3]);
        let block = Block::default().title("top").borders(Borders::ALL);
        f.render_widget(block, top_bar_rect);

        Main::new(self).render(f, main_rect, state);
        Playbar::new(self).render(f, playbar_rect, state);
        StatusBar::new(self).render(f, statusbar_rect, state);
    }
}
