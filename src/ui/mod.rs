mod draw;
mod event;

use crate::ClientResult;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use event::{EventHandler, InputEvent};
use tui::backend::CrosstermBackend;
use tui::widgets::*;
use tui::Terminal;

#[derive(Debug)]
pub struct UIState {}

impl UIState {
    pub fn new() -> Self {
        Self {}
    }
}

pub(crate) struct UI {
    pub state: UIState,
}

impl UI {
    pub fn new() -> Self {
        let state = UIState::new();
        Self { state }
    }

    pub fn start(&mut self) -> ClientResult<()> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        crossterm::terminal::enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        let event_handler = EventHandler::new();

        loop {
            terminal.draw(|mut f| {
                let size = f.size();
                let block = Block::default().title("block").borders(Borders::ALL);
                f.render_widget(block, size);
            })?;

            match event_handler.recv()? {
                InputEvent::Input(key) => {
                    if key == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
                        break;
                    }
                    trace!("{:?} pressed", key);
                }
                InputEvent::Tick => {}
            }
        }
        crossterm::terminal::disable_raw_mode().unwrap();
        Ok(())
    }
}
