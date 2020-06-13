use crate::*;
use error::DeserializationError;
use std::path::Path;

pub struct BinaryDecoder;

impl Decoder for BinaryDecoder {
    type Error = ProtocolError;
    fn decode_add_file<'a>(&mut self, buf: &'a [u8]) -> ProtocolResult<Vec<&'a Path>> {
        let paths = bincode::deserialize::<Vec<&Path>>(buf).map_err(|_| DeserializationError)?;
        dbg!(&paths);
        Ok(paths)
    }

    fn decode_opcode(&mut self, u: u8) -> Result<Opcode, Self::Error> {
        Opcode::from_u8(u)
    }

    fn decode_i32(&mut self, buf: &[u8; 4]) -> Result<i32, Self::Error> {
        Ok(i32::from_be_bytes(*buf))
    }
}
