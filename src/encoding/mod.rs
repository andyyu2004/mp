mod binary_encoder;
mod encodable;
mod encoder;

pub use binary_encoder::BinaryEncoder;
pub use encodable::Encodable;
pub use encoder::Encoder;

use crate::{error::ParseError, ProtocolResult};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

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
