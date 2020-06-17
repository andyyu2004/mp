use crate::{error::ParseError, ProtocolResult};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    // ADD_FILE <file_path> (file includes directories)
    AddFile = 0x00,
    FetchTrk = 0x01,
    PlayTrk = 0x02,
    FetchPlaybackState = 0x03,
    ResumePlayback = 0x04,
    PausePlayback = 0x05,
    TogglePlay = 0x06,
    QAppend = 0x07,
    QFetch = 0x08,
    SetNxtTrk = 0x09,
    PlayPrv = 0x0A,
    PlayNxt = 0x0B,
}

impl Opcode {
    pub fn from_u8(u: u8) -> ProtocolResult<Self> {
        let op = Self::try_from(u).map_err(|_| ParseError::InvalidOpcode(u))?;
        Ok(op)
    }
}
