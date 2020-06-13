mod components;
mod event;
pub(crate) mod handlers;
mod key;
mod region;
mod render;
mod uistate;

use crate::{Client, ClientResult};
use event::{EventHandler, InputEvent};
pub(crate) use key::Key;
use region::Region;
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
            {
                let client = self.client.lock().await;
                let uistate = &mut self.uistate;
                terminal.draw(|mut f| {
                    let size = f.size();
                    uistate.render(&mut f, size, &client.state);
                })?;
            }

            match event_handler.recv()? {
                InputEvent::Input(key) => {
                    if key == Key::Ctrl('c') {
                        break;
                    }
                    self.handle_keypress(key).await;
                }
                InputEvent::Tick => {}
            }
        }

        terminal.clear()?;
        crossterm::terminal::disable_raw_mode().unwrap();
        Ok(())
    }
}
