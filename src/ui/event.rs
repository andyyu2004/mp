use crossterm::event::{Event, KeyEvent};
use std::io;
use std::{sync::mpsc, thread, time::Duration};

const TICK_RATE: u64 = 1000;

pub(crate) enum InputEvent<T> {
    Input(T),
    Tick,
}
pub(crate) struct EventHandler {
    rx: mpsc::Receiver<InputEvent<KeyEvent>>,
    _tx: mpsc::Sender<InputEvent<KeyEvent>>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        let duration = Duration::from_millis(TICK_RATE);
        thread::spawn::<_, io::Result<()>>(move || loop {
            if crossterm::event::poll(duration).unwrap() {
                if let Event::Key(key) = crossterm::event::read().unwrap() {
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
            event_tx.send(InputEvent::Tick).unwrap();
        });

        Self { _tx: tx, rx }
    }

    pub fn recv(&self) -> Result<InputEvent<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}
