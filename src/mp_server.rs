use crate::file;
use crate::{Server, ServerResult};
use mp_protocol::Response;
use std::path::Path;

pub(crate) trait MpServer {
    fn add_files(&mut self, paths: &Vec<&Path>) -> ServerResult<Response>;
}

impl Server<'_> {
    pub(crate) fn handle_add_files(&mut self, paths: &Vec<&Path>) -> ServerResult<Response> {
        let tags = file::get_all_tags(paths)?;
        self.db.insert_files(&tags)?;
        self.handle_fetch_tracks()
    }

    pub(crate) fn handle_fetch_tracks(&mut self) -> ServerResult<Response> {
        Ok(Response::Tracks(self.db.get_all()?))
    }

    pub(crate) fn handle_play_track(&mut self, track_id: i32) -> ServerResult<Response> {
        let track = self.db.get_track(track_id)?;
        self.player.play_file(track)?;
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
