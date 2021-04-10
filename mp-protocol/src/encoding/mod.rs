mod binary_encoder;
mod encode;
mod encoder;

pub use binary_encoder::BinaryEncoder;
pub use encode::Encode;
pub use encoder::Encoder;

use crate::{error::ParseError, ProtocolResult};
use num_enum::TryFromPrimitive;
use std::{convert::TryFrom, io::Write};

pub fn binary_encode_to_bytes<T>(item: &T, writer: impl Write) -> ProtocolResult<()>
where
    T: Encode,
{
    let mut encoder = BinaryEncoder::new(writer);
    item.encode(&mut encoder)
}

/// Represents the encoding used in the binary stream sent to the server
#[repr(u8)]
#[derive(Debug, Eq, Clone, Copy, PartialEq, TryFromPrimitive)]
pub enum Encoding {
    Binary,
    Json,
}

impl Encoding {
    pub fn from_u8(u: u8) -> ProtocolResult<Self> {
        let op = Self::try_from(u).map_err(|_| ParseError::InvalidOpcode(u))?;
        Ok(op)
    }
}
