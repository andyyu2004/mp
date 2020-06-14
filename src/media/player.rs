use super::{MPState, MediaEventHandler, MediaResult};
use mp_protocol::{JoinedTrack, PlaybackState};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{collections::VecDeque, fs::File, io::BufReader, path::Path};

pub(crate) struct Player {
    instance: vlc::Instance,
    player: vlc::MediaPlayer,
    state: Arc<Mutex<MPState>>,
    tx: Sender<vlc::Event>,
}

pub(crate) trait MediaPlayer {}

impl Player {
    pub fn new(tx: Sender<vlc::Event>, state: Arc<Mutex<MPState>>) -> Self {
        let instance = vlc::Instance::new().unwrap();
        let player = vlc::MediaPlayer::new(&instance).unwrap();

        let event_manager = player.event_manager();
        // subscribe to events we care about
        for event in &[vlc::EventType::MediaPlayerEndReached] {
            let txc = tx.clone();
            event_manager
                .attach(vlc::EventType::MediaPlayerEndReached, move |event, _obj| {
                    txc.send(event).unwrap()
                })
                .unwrap();
        }
        Self {
            instance,
            player,
            tx,
            state,
        }
    }

    /// plays the audio from provided path
    /// assumes the path is valid
    /// stops any other playback and immediately plays the specified file
    pub fn play_file(&mut self, track: JoinedTrack) -> MediaResult<()> {
        let media = vlc::Media::new_path(&self.instance, &track.path).unwrap();
        self.state.lock().unwrap().push_front(track);
        self.player.set_media(&media);
        self.player.play().unwrap();
        Ok(())
    }

    pub fn get_status(&self) -> PlaybackState {
        let state = self.state.lock().unwrap();
        PlaybackState {
            curr_track: state.current_track().map(Clone::clone),
            duration: self
                .player
                .get_media()
                .and_then(|m| m.duration())
                .unwrap_or(0),
            curr_time: self.player.get_time().unwrap_or(0),
            is_playing: self.player.is_playing(),
        }
    }

    /// appends the audio from provided path to queue
    /// assumes the path is valid
    pub fn append_file_to_queue(&self, path: impl AsRef<Path>) -> MediaResult<()> {
        todo!();
    }
}
