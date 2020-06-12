mod components;
mod event;
mod render;
mod uistate;

use crate::{Client, ClientResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use event::{EventHandler, InputEvent};
use render::Render;
use std::sync::Arc;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use uistate::UIState;

pub(crate) struct UI<'a> {
    pub uistate: UIState,
    pub client: Arc<Mutex<Client<'a>>>,
}

impl<'a> UI<'a> {
    pub fn new(client: Arc<Mutex<Client<'a>>>) -> Self {
        Self {
            client,
            uistate: UIState::default(),
        }
    }

    pub async fn start(&mut self) -> ClientResult<()> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        crossterm::terminal::enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        let event_handler = EventHandler::new();

        loop {
            let client = self.client.lock().await;
            let uistate = &mut self.uistate;
            terminal.draw(|mut f| {
                let size = f.size();
                uistate.render(&mut f, size, &client.state);
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
