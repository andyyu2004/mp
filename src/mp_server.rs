use crate::file;
use crate::media::*;
use crate::{Server, ServerResult};
use mp_protocol::Response;
use std::path::Path;

pub(crate) trait MpServer {
    fn add_files(&mut self, paths: &Vec<&Path>) -> ServerResult<Response>;
}

impl Server {
    pub(crate) fn handle_add_files(&mut self, paths: &Vec<&Path>) -> ServerResult<Response> {
        let tags = file::get_all_tags(paths)?;
        self.db.insert_files(tags)?;
        self.handle_fetch_tracks()
    }

    pub(crate) fn handle_fetch_tracks(&mut self) -> ServerResult<Response> {
        Ok(Response::Tracks(self.db.get_all()?))
    }

    pub(crate) async fn handle_fetch_q(&mut self) -> ServerResult<Response> {
        let state = self.mp_state.lock().await;
        let (hist, q) = state.getq();
        Ok(Response::Q(hist.to_vec(), q.to_owned()))
    }

    pub(crate) async fn handle_play_track(&mut self, track_id: i32) -> ServerResult<Response> {
        let track = self.db.get_track(track_id)?;
        let event = MediaEvent::new(MediaResponseKind::Q, MediaEventKind::PlayTrack(track));
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    pub(crate) async fn handle_q_append(&mut self, track_id: i32) -> ServerResult<Response> {
        let track = self.db.get_track(track_id)?;
        let event = MediaEvent::new(MediaResponseKind::Q, MediaEventKind::QAppend(track));
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    /// listens for a response from the player
    async fn listen_rx(&mut self) -> ServerResult<Response> {
        match self.server_rx.recv().await.unwrap() {
            MediaPlayerData::Q(q, hist) => Ok(Response::Q(q, hist)),
            MediaPlayerData::PlaybackState(state) => Ok(Response::PlaybackState(state)),
        }
    }

    pub(crate) async fn handle_fetch_playback_state(&mut self) -> ServerResult<Response> {
        let event = MediaEvent::new(MediaResponseKind::PlaybackState, MediaEventKind::None);
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    pub(crate) async fn handle_pause_playback(&mut self) -> ServerResult<Response> {
        let event = MediaEvent::new(MediaResponseKind::None, MediaEventKind::Pause);
        self.mp_tx.send(event).await.unwrap();
        Ok(Response::Ok)
    }

    pub(crate) async fn handle_toggle_play(&mut self) -> ServerResult<Response> {
        let event = MediaEvent::new(MediaResponseKind::None, MediaEventKind::TogglePlay);
        self.mp_tx.send(event).await.unwrap();
        Ok(Response::Ok)
    }

    pub(crate) async fn handle_resume_playback(&mut self) -> ServerResult<Response> {
        let event = MediaEvent::new(MediaResponseKind::None, MediaEventKind::Resume);
        self.mp_tx.send(event).await.unwrap();
        Ok(Response::Ok)
    }

    pub(crate) async fn handle_play_prev(&mut self) -> ServerResult<Response> {
        let event = MediaEvent::new(MediaResponseKind::Q, MediaEventKind::PlayPrev);
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    pub(crate) async fn handle_play_next(&mut self) -> ServerResult<Response> {
        let event = MediaEvent::new(MediaResponseKind::Q, MediaEventKind::PlayNext);
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    pub(crate) async fn handle_shuffle_all(&mut self) -> ServerResult<Response> {
        let tracks = self.db.get_all()?;
        let event = MediaEvent::new(MediaResponseKind::Q, MediaEventKind::ShuffleAll(tracks));
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    pub(crate) async fn handle_set_next_track(&mut self, track_id: i32) -> ServerResult<Response> {
        let track = self.db.get_track(track_id)?;
        let event = MediaEvent::new(MediaResponseKind::Q, MediaEventKind::SetNextTrack(track));
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }

    pub(crate) async fn handle_seek(&mut self, seek_amount: i64) -> ServerResult<Response> {
        let event = MediaEvent::new(
            MediaResponseKind::PlaybackState,
            MediaEventKind::Seek(seek_amount),
        );
        self.mp_tx.send(event).await.unwrap();
        self.listen_rx().await
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn result_flat_map() {
        fn f(x: i32) -> Result<Vec<i32>, ()> {
            if x < 50 {
                Ok(vec![x - 1, x + 1])
            } else {
                Err(())
            }
        }

        let xs: Vec<_> = vec![10, 40, 70].into_iter().flat_map(f).collect();
        // flattening results just removes the failures
        assert_eq!(xs, vec![vec![9, 11], vec![39, 41]]);

        // flatmap with flatten works nice
        let xs: Vec<_> = vec![10, 40, 70].into_iter().flat_map(f).flatten().collect();
        assert_eq!(xs, vec![9, 11, 39, 41]);

        // collection fails the entire operation
        let xs = vec![10, 40, 70].into_iter().map(f).collect::<Result<Vec<_>, _>>();
        assert_eq!(xs, Err(()));
    }
}
