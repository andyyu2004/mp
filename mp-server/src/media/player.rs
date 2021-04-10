use super::*;
use futures::executor::block_on;
use mp_protocol::{JoinedTrack, PlaybackState};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use vlc::MediaPlayerAudioEx;

/// the data the player sends the server (i.e. playback state etc.)
#[derive(Debug)]
pub(crate) enum MediaPlayerData {
    PlaybackState(PlaybackState),
    Queue(Vec<JoinedTrack>, VecDeque<JoinedTrack>),
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

        Self { instance, player, media_rx, media_tx, server_tx, state }
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
                MediaEventKind::ShuffleAll(tracks) => self.shuffle_all(tracks).await,
                MediaEventKind::SetNextTrack(track) => self.set_next_track(track).await,
                MediaEventKind::PlayTrack(track) => self.play_immediate(track).await,
                MediaEventKind::QueueAppend(track) => self.queue_append(track).await,
                MediaEventKind::Seek(delta) => self.seek(delta),
                MediaEventKind::ChangeVolume(delta) => self.change_volume(delta),
                MediaEventKind::PlayPrev => self.play_prev().await,
                MediaEventKind::Pause => self.player.set_pause(true),
                MediaEventKind::Resume => self.player.set_pause(false),
                MediaEventKind::TogglePlay => self.player.pause(),
                MediaEventKind::PlayNext => self.play_next().await,
                MediaEventKind::None => {}
            };

            match event.expected_response {
                MediaResponseKind::None => {}
                MediaResponseKind::PlaybackState => self.send_status().await,
                MediaResponseKind::Queue => self.send_q().await,
            };
        }
    }

    pub async fn set_next_track(&mut self, track: JoinedTrack) {
        let mut state = self.state.lock().await;
        state.set_next_track(track);
    }

    pub fn change_volume(&mut self, delta: i32) {
        self.player.set_volume(self.player.get_volume() + delta).unwrap();
    }

    pub fn seek(&mut self, seek_amt: i64) {
        if let Some(curr_time) = self.player.get_time() {
            self.player.set_time(std::cmp::max(0, curr_time + seek_amt));
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
        let mut state = self.state.lock().await;
        state.set_next_track(track);
        state.play_next().map(|track| self.play_track(track));
    }

    pub fn play_track(&self, track: &JoinedTrack) {
        let media = vlc::Media::new_path(&self.instance, &track.path).unwrap();
        // if some parameters are required
        // let cstr = std::ffi::CString::new("vlc parameters".as_bytes()).unwrap();
        // unsafe { vlc::sys::libvlc_media_add_option(media.raw(), cstr.as_ptr()); }
        self.player.set_media(&media);
        self.player.set_volume(75).unwrap();
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
            volume: self.player.get_volume(),
        }
    }

    async fn send_status(&mut self) {
        let playback_state = self.get_status().await;
        let data = MediaPlayerData::PlaybackState(playback_state);
        self.server_tx.send(data).await.unwrap();
    }

    async fn send_q(&mut self) {
        let state = self.state.lock().await;
        let (q, hist) = state.get_queue();
        let data = MediaPlayerData::Queue(q.to_owned(), hist.to_owned());
        self.server_tx.send(data).await.unwrap();
    }

    pub async fn queue_append(&mut self, track: JoinedTrack) {
        let mut state = self.state.lock().await;
        state.append(track);
    }
}
