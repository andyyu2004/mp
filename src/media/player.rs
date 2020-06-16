use super::{MPState, MediaEvent, MediaEventHandler, MediaResult};
use mp_protocol::{JoinedTrack, PlaybackState};
use std::sync::{Arc, Mutex};
use std::{collections::VecDeque, fs::File, io::BufReader, path::Path};
use tokio::sync::mpsc::{Receiver, Sender};

pub(crate) struct Player {
    instance: vlc::Instance,
    player: vlc::MediaPlayer,
    state: Arc<Mutex<MPState>>,
    rx: Receiver<MediaEvent>,
}

pub(crate) trait MediaPlayer {}

impl Player {
    pub fn new(rx: Receiver<MediaEvent>, state: Arc<Mutex<MPState>>) -> Self {
        let instance = vlc::Instance::new().unwrap();
        let player = vlc::MediaPlayer::new(&instance).unwrap();

        let event_manager = player.event_manager();
        // subscribe to events we care about
        // for &event_type in &[vlc::EventType::MediaPlayerEndReached] {
        //     let txc = tx.clone();
        //     event_manager
        //         .attach(event_type, move |event, _obj| txc.send(event).unwrap())
        //         .unwrap();
        //     }
        Self {
            instance,
            player,
            rx,
            state,
        }
    }

    pub async fn listen(&mut self) -> MediaResult<()> {
        while let Some(event) = self.rx.recv().await {
            match event {
                MediaEvent::Pause => Ok(self.pause()),
                MediaEvent::Resume => Ok(self.resume()),
                MediaEvent::TogglePlay => Ok(self.toggle_play()),
                MediaEvent::PlayTrack(track) => self.play_track(track),
            }?;
        }
        Ok(())
    }

    /// plays the audio from provided path
    /// assumes the path is valid
    /// stops any other playback and immediately plays the specified file
    pub fn play_track(&mut self, track: JoinedTrack) -> MediaResult<()> {
        let media = vlc::Media::new_path(&self.instance, &track.path).unwrap();
        self.state.lock().unwrap().push_front(track);
        self.player.set_media(&media);
        self.player.play().unwrap();
        Ok(())
    }

    pub fn pause(&mut self) {
        self.player.set_pause(true)
    }

    pub fn resume(&mut self) {
        self.player.set_pause(false)
    }

    pub fn toggle_play(&mut self) {
        self.player.pause()
    }

    pub fn getq(&mut self) -> (Vec<JoinedTrack>, VecDeque<JoinedTrack>) {
        let state = self.state.lock().unwrap();
        let (hist, q) = state.get();
        (hist.clone(), q.clone())
    }

    pub fn get_status(&self) -> PlaybackState {
        let state = self.state.lock().unwrap();
        PlaybackState {
            curr_track: state.current_track().map(Clone::clone),
            duration: self
                .player
                .get_media()
                .and_then(|m| m.duration())
                .unwrap_or(1),
            progress: self.player.get_time().unwrap_or(0),
            is_playing: self.player.is_playing(),
        }
    }

    /// appends the audio from provided path to queue
    /// assumes the path is valid
    pub fn q_append(&self, track: JoinedTrack) {
        let mut state = self.state.lock().unwrap();
        state.append(track)
    }
}
