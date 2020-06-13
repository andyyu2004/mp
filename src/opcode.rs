use crate::{error::ParseError, ProtocolResult};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    // ADD_FILE <file_path> (file includes directories)
    AddFile = 0x00,
    FetchTracks = 0x01,
    PlayTrack = 0x02,
}

impl Opcode {
    pub fn from_u8(u: u8) -> ProtocolResult<Self> {
        let op = Self::try_from(u).map_err(|_| ParseError::InvalidOpcode(u))?;
        Ok(op)
    }
}
