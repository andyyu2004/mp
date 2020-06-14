use super::Connection;
use mp_protocol::{JoinedTrack, ProtocolResult, Request, Response, RES_BUF_CAP};
use std::path::Path;

impl Connection {
    pub(crate) async fn recv_response(&mut self) -> ProtocolResult<Response> {
        let mut buf = vec![0u8; RES_BUF_CAP];
        let count = self.recv(&mut buf).await?;
        let response = serde_json::from_slice::<Response>(&buf[..count])?;
        Ok(response)
    }

    /// sends the request, receives the response, and handles appropriately
    pub(crate) async fn send_recv_handle<'a>(
        &mut self,
        req: &'a Request<'a>,
    ) -> ProtocolResult<()> {
        self.send_request(req).await?;
        let res = self.recv_response().await?;
        self.handle_response(res).await
    }

    pub(crate) async fn handle_response(&mut self, response: Response) -> ProtocolResult<()> {
        let mut client = self.client.lock().unwrap();
        Ok(match response {
            Response::Ok => (),
            Response::Tracks(tracks) => client.state.tracks = tracks,
            Response::PlaybackState(playback_state) => client.state.playback_state = playback_state,
            Response::Error => panic!("server sent back error"),
        })
    }

    pub(crate) async fn send_request<'a>(&mut self, req: &'a Request<'a>) -> ProtocolResult<()> {
        let mut buf = vec![];
        mp_protocol::binary_encode_to_bytes(req, &mut buf)?;
        self.send(&buf).await?;
        Ok(())
    }

    pub(crate) async fn fetch_playback_state(&mut self) -> ProtocolResult<()> {
        self.send_recv_handle(&Request::FetchPlaybackState).await
    }

    pub(crate) async fn fetch_tracks(&mut self) -> ProtocolResult<()> {
        self.send_recv_handle(&Request::FetchTracks).await
    }

    pub(crate) async fn add_files(&mut self, files: Vec<&Path>) -> ProtocolResult<()> {
        self.send_recv_handle(&Request::AddFile(files)).await
    }

    pub async fn play_track(&mut self, track_id: i32) -> ProtocolResult<()> {
        self.send_recv_handle(&Request::PlayTrack(track_id)).await
    }
}
