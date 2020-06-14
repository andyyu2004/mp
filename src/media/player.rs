use super::MediaResult;
use mp_protocol::{JoinedTrack, PlaybackState};
use std::{fs::File, io::BufReader, path::Path};

pub(crate) struct Player {
    instance: vlc::Instance,
    player: vlc::MediaPlayer,
    curr_track: Option<JoinedTrack>,
}

pub(crate) trait MediaPlayer {}

impl Player {
    pub fn new() -> Self {
        let instance = vlc::Instance::new().unwrap();
        let player = vlc::MediaPlayer::new(&instance).unwrap();
        Self {
            instance,
            player,
            curr_track: None,
        }
    }

    /// plays the audio from provided path
    /// assumes the path is valid
    /// stops any other playback and immediately plays the specified file
    pub fn play_file(&mut self, track: JoinedTrack) -> MediaResult<()> {
        let media = vlc::Media::new_path(&self.instance, &track.path).unwrap();
        self.player.set_media(&media);
        self.player.play().unwrap();
        Ok(())
    }

    pub fn get_status(&self) -> PlaybackState {
        PlaybackState {
            curr_track: self.curr_track.clone(),
            duration: self
                .player
                .get_media()
                .and_then(|m| m.duration())
                .unwrap_or(0),
            curr_time: self.player.get_time().unwrap_or(0),
            is_playing: self.player.is_playing(),
        }
    }

    /// appends the audio from provided path to queue
    /// assumes the path is valid
    pub fn append_file_to_queue(&self, path: impl AsRef<Path>) -> MediaResult<()> {
        todo!();
    }
}
