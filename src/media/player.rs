use super::{MPState, MediaEvent, MediaEventHandler, MediaResult};
use futures::executor::block_on;
use mp_protocol::{JoinedTrack, PlaybackState};
use std::sync::Arc;
use std::{collections::VecDeque, fs::File, io::BufReader, path::Path};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

/// the data the player sends the server (i.e. playback state etc.)
#[derive(Debug)]
pub(crate) enum MediaPlayerData {
    PlaybackState(PlaybackState),
}

pub(crate) struct Player {
    instance: vlc::Instance,
    player: vlc::MediaPlayer,
    state: Arc<Mutex<MPState>>,
    media_tx: Sender<MediaEvent>,
    media_rx: Receiver<MediaEvent>,
    server_tx: Sender<MediaPlayerData>,
}

pub(crate) trait MediaPlayer {}

impl Player {
    pub fn new(
        media_tx: Sender<MediaEvent>,
        media_rx: Receiver<MediaEvent>,
        server_tx: Sender<MediaPlayerData>,
        state: Arc<Mutex<MPState>>,
    ) -> Self {
        let instance = vlc::Instance::new().unwrap();
        let player = vlc::MediaPlayer::new(&instance).unwrap();

        Self::subscribe_vlc_events(media_tx.clone(), &mut player.event_manager());

        Self {
            instance,
            player,
            media_rx,
            media_tx,
            server_tx,
            state,
        }
    }

    fn subscribe_vlc_events(tx: Sender<MediaEvent>, event_manager: &mut vlc::EventManager) {
        event_manager
            .attach(vlc::EventType::MediaPlayerEndReached, move |event, _obj| {
                block_on(tx.clone().send(MediaEvent::PlayNext)).unwrap();
            })
            .unwrap();
    }

    pub async fn listen(&mut self) {
        while let Some(event) = self.media_rx.recv().await {
            match event {
                MediaEvent::Pause => self.pause(),
                MediaEvent::Resume => self.resume(),
                MediaEvent::TogglePlay => self.toggle_play(),
                MediaEvent::PlayNext => self.play_next(),
                MediaEvent::PlaybackState => self.send_status().await,
                MediaEvent::PlayTrack(track) => self.play_track(track).await,
            };
        }
    }

    /// stops any other playback and immediately plays the specified track
    pub async fn play_track(&mut self, track: JoinedTrack) {
        let media = vlc::Media::new_path(&self.instance, &track.path).unwrap();
        self.state.lock().await.push_front(track);
        self.player.set_media(&media);
        self.player.play().unwrap();
    }

    fn pause(&mut self) {
        self.player.set_pause(true)
    }

    fn resume(&mut self) {
        self.player.set_pause(false)
    }

    fn toggle_play(&mut self) {
        self.player.pause()
    }

    fn play_next(&self) {
    }

    async fn send_status(&mut self) {
        let playback_state = self.get_status().await;
        let data = MediaPlayerData::PlaybackState(playback_state);
        self.server_tx.send(data).await.unwrap();
    }

    async fn get_status(&self) -> PlaybackState {
        let state = self.state.lock().await;
        PlaybackState {
            curr_track: state.curr_track().map(Clone::clone),
            progress: self.player.get_time().unwrap_or(0),
            is_playing: self.player.is_playing(),
        }
    }

    /// appends the audio from provided path to queue
    /// assumes the path is valid
    pub async fn q_append(&self, track: JoinedTrack) {
        let mut state = self.state.lock().await;
        state.append(track)
    }
}
