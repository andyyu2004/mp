use super::key::Key;
use crossterm::event::Event;
use std::io;
use std::{sync::mpsc, thread, time::Duration};

const TICK_RATE: u64 = 100;

pub(crate) enum InputEvent<T> {
    Input(T),
    Tick,
}
pub(crate) struct EventHandler {
    rx: mpsc::Receiver<InputEvent<Key>>,
    _tx: mpsc::Sender<InputEvent<Key>>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        let duration = Duration::from_millis(TICK_RATE);
        thread::spawn::<_, io::Result<()>>(move || loop {
            if crossterm::event::poll(duration).unwrap() {
                if let Event::Key(key_event) = crossterm::event::read().unwrap() {
                    let key = Key::from(key_event);
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
            event_tx.send(InputEvent::Tick).unwrap();
        });

        Self { _tx: tx, rx }
    }

    pub fn recv(&self) -> Result<InputEvent<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
