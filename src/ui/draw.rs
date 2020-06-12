use super::UI;
use tui::backend::Backend;
use tui::layout::*;
use tui::widgets::*;
use tui::Frame;

pub(crate) trait Draw {
    fn get_layout(&self) -> Layout;

    fn draw_main_layout<B>(&self, f: &mut Frame<B>)
    where
        B: Backend;
}

impl Draw for UI {
    fn get_layout(&self) -> Layout {
        let constraints = [
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(6),
        ];

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
    }

    fn draw_main_layout<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        self.get_layout().split(f.size());
    }
}
