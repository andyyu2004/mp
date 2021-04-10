use crate::error::ParseError;
use crate::ProtocolResult;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(non_snake_case)]
pub enum Opcode {
    // ADD_FILE <file_path> (file includes directories)
    AddFile    = 0x00,
    FetchTrack = 0x01,
    PlayTrack  = 0x02,
    FetchPlaybackState = 0x03,
    ResumePlayback = 0x04,
    PausePlayback = 0x05,
    TogglePlay = 0x06,
    QueueAppend = 0x07,
    FetchQueue = 0x08,
    SetNextTrack = 0x09,
    PlayPrev   = 0x0A,
    PlayNext   = 0x0B,
    ShuffleAll = 0x0C,
    Seek       = 0x0D,
    ChangeVolume = 0x0E,
}

impl Default for Opcode {
    fn default() -> Self {
        Self::Seek
    }
}

impl Opcode {
    pub fn from_u8(u: u8) -> ProtocolResult<Self> {
        let op = Self::try_from(u).map_err(|_| ParseError::InvalidOpcode(u))?;
        Ok(op)
    }
}
