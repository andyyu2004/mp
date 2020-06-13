use std::path::Path;

pub trait Encoder {
    type Ok;
    type Error;

    fn encode_fetch_tracks(&mut self) -> Result<Self::Ok, Self::Error>;
    fn encode_play_track(&mut self, track_id: i32) -> Result<Self::Ok, Self::Error>;

    fn encode_add_file(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self::Ok, Self::Error>;
}
