mod components;
mod event;
pub(crate) mod handlers;
mod key;
mod region;
mod render;
mod uistate;

use crate::{network::IOEvent, Client, ClientResult};
use event::{EventHandler, InputEvent};
pub(crate) use key::Key;
pub(crate) use region::Region;
use render::Render;
use std::sync::{mpsc::Sender, Arc, Mutex};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use uistate::UIState;

pub(crate) struct UI {
    pub uistate: UIState,
    pub client: Arc<Mutex<Client>>,
    pub io_tx: Sender<IOEvent>,
}

impl UI {
    pub fn new(client: Arc<Mutex<Client>>, io_tx: Sender<IOEvent>) -> Self {
        Self {
            client,
            uistate: UIState::default(),
            io_tx,
        }
    }

    fn dispatch(&self, event: IOEvent) {
        self.io_tx.send(event).unwrap()
    }

    pub fn start(&mut self) -> ClientResult<()> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        crossterm::terminal::enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        let event_handler = EventHandler::new();
        self.dispatch(IOEvent::InitClient);

        loop {
            {
                let client = self.client.lock().unwrap();
                let uistate = &mut self.uistate;
                if uistate.should_quit {
                    break;
                }
                terminal.draw(|mut f| {
                    let size = f.size();
                    uistate.render(&mut f, size, &client.state);
                })?;
            }

            match event_handler.recv()? {
                InputEvent::Input(Key::Ctrl('z')) => break,
                InputEvent::Input(key) => self.handle_keypress(key),
                InputEvent::Tick => self.tick(),
            };
        }

        crossterm::terminal::disable_raw_mode().unwrap();
        self.dispatch(IOEvent::Terminate);
        terminal.clear()?;
        Ok(())
    }

    fn tick(&self) {
        self.io_tx.send(IOEvent::UpdatePlaybackStatus).unwrap();
    }
}
