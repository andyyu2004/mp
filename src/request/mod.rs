use crate::decoding;
use crate::{Decode, Decoder, Encoding, Opcode, ProtocolError, ProtocolResult};
use std::convert::TryFrom;
use std::path::Path;

/// request is an enumeration of all requests that a client can send to the server
#[derive(Debug, PartialEq)]
pub enum Request<'r> {
    AddFile(Vec<&'r Path>),
    FetchTracks,
}

/// implement decoding of a request from bytes of any encoding (encoding is encoded in the first byte of the buffer)
impl<'r> TryFrom<&'r [u8]> for Request<'r> {
    type Error = ProtocolError;
    fn try_from(buf: &'r [u8]) -> ProtocolResult<Self> {
        let encoding = Encoding::from_u8(buf[0])?;
        let mut decoder = decoding::get_decoder(encoding);
        Request::decode(&buf[1..], decoder.as_mut())
    }
}

impl<'r> Decode<'r> for Request<'r> {
    fn decode<D>(buf: &'r [u8], mut decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder,
    {
        let opcode = decoder.decode_opcode(buf[0])?;
        Ok(match opcode {
            Opcode::AddFile => Self::AddFile(decoder.decode_add_file(&buf[1..])?),
            Opcode::FetchTracks => Self::FetchTracks,
        })
    }
}
