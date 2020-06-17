use super::*;
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
    Q(Vec<JoinedTrack>, VecDeque<JoinedTrack>),
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
        // transmitter for sending back information to server
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
            .attach(vlc::EventType::MediaPlayerEndReached, move |_event, _obj| {
                let event = MediaEvent::new(MediaResponseKind::None, MediaEventKind::PlayNext);
                block_on(tx.clone().send(event)).unwrap();
            })
            .unwrap();
    }

    pub async fn listen(&mut self) {
        while let Some(event) = self.media_rx.recv().await {
            match event.kind {
                MediaEventKind::Pause => self.player.set_pause(true),
                MediaEventKind::Resume => self.player.set_pause(false),
                MediaEventKind::TogglePlay => self.player.pause(),
                MediaEventKind::PlayNext => self.play_next().await,
                MediaEventKind::PlayTrack(track) => self.play_immediate(track).await,
                MediaEventKind::QAppend(track) => self.q_append(track).await,
                MediaEventKind::PlayPrev => self.play_prev().await,
                MediaEventKind::ShuffleAll(tracks) => self.shuffle_all(tracks).await,
                MediaEventKind::SetNextTrack(_) => {}
                MediaEventKind::None => {}
            };

            match event.expected_response {
                MediaResponseKind::None => {}
                MediaResponseKind::PlaybackState => self.send_status().await,
                MediaResponseKind::Q => self.send_q().await,
            };
        }
    }

    /// sets the queue to be a random permutation of the tracks passed in
    /// history remains unchanged
    pub async fn shuffle_all(&mut self, tracks: Vec<JoinedTrack>) {
        let mut state = self.state.lock().await;
        let track = state.shuffle_all(tracks);
        track.map(|t| self.play_track(t));
    }

    /// stops any other playback and immediately plays the specified track
    pub async fn play_immediate(&mut self, track: JoinedTrack) {
        self.play_track(&track);
        self.state.lock().await.push_front(track);
    }

    pub fn play_track(&self, track: &JoinedTrack) {
        let media = vlc::Media::new_path(&self.instance, &track.path).unwrap();
        self.player.set_media(&media);
        self.player.play().unwrap();
    }

    async fn play_prev(&mut self) {
        match self.player.get_time() {
            Some(t) if t > 2000 => self.replay().await,
            _ => {
                let mut state = self.state.lock().await;
                state.play_prev().map(|t| self.play_track(t));
            }
        };
    }

    async fn replay(&mut self) {
        self.player.set_time(0)
    }

    async fn play_next(&mut self) {
        let mut state = self.state.lock().await;
        state.play_next().map(|t| self.play_track(t));
    }

    async fn get_status(&self) -> PlaybackState {
        let state = self.state.lock().await;
        PlaybackState {
            curr_track: state.curr_track().map(Clone::clone),
            progress: self.player.get_time().unwrap_or(0),
            is_playing: self.player.is_playing(),
        }
    }

    async fn send_status(&mut self) {
        let playback_state = self.get_status().await;
        let data = MediaPlayerData::PlaybackState(playback_state);
        self.server_tx.send(data).await.unwrap();
    }

    async fn send_q(&mut self) {
        let state = self.state.lock().await;
        let (q, hist) = state.getq();
        let data = MediaPlayerData::Q(q.to_owned(), hist.to_owned());
        self.server_tx.send(data).await.unwrap();
    }

    pub async fn q_append(&mut self, track: JoinedTrack) {
        let mut state = self.state.lock().await;
        state.append(track);
    }
}
