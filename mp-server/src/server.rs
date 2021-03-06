use crate::media::*;
use crate::Database;
use mp_protocol::{Request, Response};
use std::io;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

pub(crate) struct Server {
    pub(crate) db: Database,
    pub(crate) mp_tx: Sender<MediaEvent>,
    pub(crate) server_rx: Receiver<MediaPlayerData>,
    pub(crate) mp_state: Arc<Mutex<MPState>>,
}

impl Server {
    pub fn new(
        mp_tx: Sender<MediaEvent>,
        server_rx: Receiver<MediaPlayerData>,
        mp_state: Arc<Mutex<MPState>>,
    ) -> io::Result<Self> {
        Ok(Self { db: Database::new(), mp_tx, server_rx, mp_state })
    }

    pub async fn handle_request(&mut self, req: Request<'_>) -> Response {
        let res = match req {
            Request::AddFile(paths) => self.handle_add_files(&paths),
            Request::PlayTrack(track_id) => self.handle_play_track(track_id).await,
            Request::QueueAppend(track_id) => self.handle_queue_append(track_id).await,
            Request::SetNextTrack(track_id) => self.handle_set_next_track(track_id).await,
            Request::ChangeVolume(delta) => self.handle_change_volume(delta).await,
            Request::Seek(t) => self.handle_seek(t).await,
            Request::Canonicalize(src, dest) => self.handle_canonicalize(src, dest).await,
            Request::FetchTracks => self.handle_fetch_tracks(),
            Request::FetchPlaybackState => self.handle_fetch_playback_state().await,
            Request::PausePlayback => self.handle_pause_playback().await,
            Request::ResumePlayback => self.handle_resume_playback().await,
            Request::TogglePlay => self.handle_toggle_play().await,
            Request::FetchQueue => self.handle_fetch_queue().await,
            Request::PlayPrev => self.handle_play_prev().await,
            Request::PlayNext => self.handle_play_next().await,
            Request::ShuffleAll => self.handle_shuffle_all().await,
        };

        match res {
            Ok(res) => res,
            Err(err) => {
                println!("handle response error: {}", err);
                Response::Error
            }
        }
    }
}
