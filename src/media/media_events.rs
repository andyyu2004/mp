use super::MPState;
use std::sync::{mpsc::Receiver, Arc, Mutex};
use vlc::{Event, EventType, VLCObject};

pub(crate) struct MediaEventHandler {
    mp_state: Arc<Mutex<MPState>>,
    rx: Receiver<vlc::Event>,
}

impl MediaEventHandler {
    // listens for vlc events and performs the appropriate modifications on mpstate
    pub fn listen(&self) {
        while let Ok(event) = self.rx.recv() {
            let mpstate = self.mp_state.lock().unwrap();
            match event {
                vlc::Event::MediaPlayerEndReached => println!("recv event {:?}", event),
                _ => {}
            }
        }
    }
    pub fn new(mp_state: Arc<Mutex<MPState>>, rx: Receiver<vlc::Event>) -> Self {
        Self { mp_state, rx }
    }

    pub fn handle_end_reached(&mut self, event_type: Event, obj: VLCObject) {
        dbg!("yeetttt");
    }
}
