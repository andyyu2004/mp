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
