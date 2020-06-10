use crate::decoding;
use crate::{BinaryDecoder, Decodable, Decoder, Encoding, Opcode, ProtocolError, ProtocolResult};
use std::{convert::TryFrom, path::Path};

#[derive(Debug, PartialEq)]
pub enum Request<'r> {
    AddFile(Vec<&'r Path>),
}

impl<'r> TryFrom<&'r [u8]> for Request<'r> {
    type Error = ProtocolError;
    fn try_from(buf: &'r [u8]) -> ProtocolResult<Self> {
        let encoding = Encoding::from_u8(buf[0])?;
        let mut decoder = decoding::get_decoder(encoding);
        Self::decode(&buf[1..], decoder.as_mut())
    }
}

impl<'r> Decodable<'r> for Request<'r> {
    fn decode<D: ?Sized>(buf: &'r [u8], decoder: &mut D) -> Result<Self, D::Error>
    where
        D: Decoder,
    {
        let opcode = decoder.decode_opcode(buf[0])?;
        Ok(match opcode {
            Opcode::AddFile => Self::AddFile(decoder.decode_add_file(&buf[1..])?),
        })
    }
}
