use super::{Connection, IOEvent};
use mp_protocol::FIN_BYTES;
use mp_protocol::{ProtocolResult, Request, Response};
use std::path::Path;

impl Connection {
    pub async fn close(&mut self) -> ProtocolResult<()> {
        self.send(&FIN_BYTES).await?;
        Ok(())
    }

    pub async fn recv_response(&mut self) -> ProtocolResult<Response> {
        let buf = self.recv().await?;
        let response = serde_json::from_slice::<Response>(&buf)?;
        Ok(response)
    }

    /// sends the request, receives the response, and handles appropriately
    pub async fn dispatch<'a>(&mut self, req: &'a Request<'a>) -> ProtocolResult<()> {
        self.send_request(req).await?;
        let res = self.recv_response().await?;
        self.handle_response(res).await
    }

    pub async fn handle_response(&mut self, response: Response) -> ProtocolResult<()> {
        let mut client = self.client.lock().unwrap();
        Ok(match response {
            Response::Ok => (),
            Response::Tracks(tracks) => client.state.tracks = tracks,
            Response::PlaybackState(playback_state) => {
                // if the current_track has changed, refresh the queue
                let s = &mut client.state.playback_state;
                if s.curr_track != playback_state.curr_track {
                    self.tx.send(IOEvent::FetchQ).unwrap();
                }
                *s = playback_state;
            }
            Response::Q(hist, q) => {
                client.state.queue = q;
                client.state.history = hist;
            }
            Response::Error => panic!("server sent back error"),
        })
    }

    pub async fn send_request<'a>(&mut self, req: &'a Request<'a>) -> ProtocolResult<()> {
        let mut buf = vec![];
        mp_protocol::binary_encode_to_bytes(req, &mut buf)?;
        self.send(&buf).await?;
        Ok(())
    }

    pub async fn dispatch_shuffle_all(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::ShuffleAll).await
    }

    pub async fn dispatch_play_prev(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::PlayPrev).await
    }

    pub async fn dispatch_play_next(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::PlayNext).await
    }

    pub async fn dispatch_set_next_track(&mut self, track_id: i32) -> ProtocolResult<()> {
        self.dispatch(&Request::SetNextTrack(track_id)).await
    }

    pub async fn dispatch_fetch_q(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::FetchQ).await
    }

    pub async fn dispatch_fetch_playback_state(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::FetchPlaybackState).await
    }

    pub async fn dispatch_fetch_tracks(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::FetchTracks).await
    }

    pub async fn dispatch_add_files(&mut self, files: Vec<&Path>) -> ProtocolResult<()> {
        self.dispatch(&Request::AddFile(files)).await
    }

    pub async fn dispatch_play_track(&mut self, track_id: i32) -> ProtocolResult<()> {
        self.dispatch(&Request::PlayTrack(track_id)).await
    }

    pub async fn dispatch_queue_append(&mut self, track_id: i32) -> ProtocolResult<()> {
        self.dispatch(&Request::QAppend(track_id)).await
    }

    pub async fn dispatch_seek(&mut self, t: i64) -> ProtocolResult<()> {
        self.dispatch(&Request::Seek(t)).await
    }

    pub async fn dispatch_pause(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::PausePlayback).await
    }

    pub async fn dispatch_play(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::TogglePlay).await
    }

    pub async fn dispatch_toggle_play(&mut self) -> ProtocolResult<()> {
        self.dispatch(&Request::TogglePlay).await
    }
}
