use crate::file;
use crate::{media::MediaEvent, Server, ServerResult};
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

    pub(crate) fn handle_fetch_q(&mut self) -> ServerResult<Response> {
        // let (hist, q) = self.player.getq();
        // Ok(Response::Q(hist, q))
        todo!()
    }

    pub(crate) async fn handle_play_track(&mut self, track_id: i32) -> ServerResult<Response> {
        let track = self.db.get_track(track_id)?;
        self.mp_tx.send(MediaEvent::PlayTrack(track)).await.unwrap();
        Ok(Response::Ok)
    }

    pub(crate) fn handle_q_append(&mut self, track_id: i32) -> ServerResult<Response> {
        let track = self.db.get_track(track_id)?;
        // self.player.q_append(track);
        Ok(Response::Ok)
    }

    pub(crate) fn handle_fetch_playback_state(&mut self) -> ServerResult<Response> {
        todo!();
        // Ok(Response::PlaybackState(self.player.get_status()))
    }

    pub(crate) async fn handle_pause_playback(&mut self) -> ServerResult<Response> {
        self.mp_tx.send(MediaEvent::Pause).await.unwrap();
        Ok(Response::Ok)
    }

    pub(crate) async fn handle_toggle_play(&mut self) -> ServerResult<Response> {
        self.mp_tx.send(MediaEvent::TogglePlay).await.unwrap();
        Ok(Response::Ok)
    }

    pub(crate) async fn handle_resume_playback(&mut self) -> ServerResult<Response> {
        // self.player.resume();
        self.mp_tx.send(MediaEvent::Resume).await.unwrap();
        Ok(Response::Ok)
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
        let xs = vec![10, 40, 70]
            .into_iter()
            .map(f)
            .collect::<Result<Vec<_>, _>>();
        assert_eq!(xs, Err(()));
    }
}
