use crate::ui::render::Render;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
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
        let hist = state.history.iter().map(|t| Text::raw(&t.title));

        let hist_widget = List::new(hist)
            .block(Block::default().title("history").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let q = state.queue.iter().map(|t| Text::raw(&t.title));

        let queue_widget = List::new(q)
            .block(Block::default().title("queue").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(rect);

        f.render_widget(hist_widget, layout[0]);
        f.render_widget(queue_widget, layout[1]);
    }
}
