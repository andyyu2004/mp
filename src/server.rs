use crate::media::{MediaEvent, Player};
use crate::Database;
use crate::ServerResult;
use mp_protocol::{Request, Response, BUF_CAP};
use std::convert::TryFrom;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc::Sender;

pub(crate) struct Server {
    pub(crate) db: Database,
    pub(crate) mp_tx: Sender<MediaEvent>,
}

impl Server {
    pub fn new(mp_tx: Sender<MediaEvent>) -> io::Result<Self> {
        Ok(Self {
            db: Database::new(),
            mp_tx,
        })
    }

    pub async fn handle_request(&mut self, req: Request<'_>) -> Response {
        let res = match req {
            Request::AddFile(paths) => self.handle_add_files(&paths),
            Request::FetchTracks => self.handle_fetch_tracks(),
            Request::PlayTrack(track_id) => self.handle_play_track(track_id).await,
            Request::QAppend(track_id) => self.handle_q_append(track_id),
            Request::FetchPlaybackState => self.handle_fetch_playback_state(),
            Request::PausePlayback => self.handle_pause_playback().await,
            Request::ResumePlayback => self.handle_resume_playback().await,
            Request::TogglePlay => self.handle_toggle_play().await,
            Request::FetchQ => self.handle_fetch_q(),
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
