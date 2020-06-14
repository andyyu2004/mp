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

    pub(crate) async fn send_request<'a>(&mut self, req: &'a Request<'a>) -> ProtocolResult<()> {
        let mut buf = vec![];
        mp_protocol::binary_encode_to_bytes(req, &mut buf)?;
        self.send(&buf).await?;
        Ok(())
    }

    pub(crate) async fn fetch_tracks(&mut self) -> ProtocolResult<Vec<JoinedTrack>> {
        self.send_request(&Request::FetchTracks).await?;
        match self.recv_response().await? {
            Response::Tracks(tracks) => Ok(tracks),
            _ => unreachable!(),
        }
    }

    pub(crate) async fn add_files(&mut self, files: Vec<&Path>) -> ProtocolResult<Response> {
        self.send_request(&Request::AddFile(files)).await?;
        self.recv_response().await
    }

    pub async fn play_track(&mut self, track_id: i32) -> ProtocolResult<Response> {
        self.send_request(&Request::PlayTrack(track_id)).await?;
        self.recv_response().await
    }
}
