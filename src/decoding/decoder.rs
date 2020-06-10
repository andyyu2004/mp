use crate::Opcode;
use std::path::Path;

pub trait Decoder {
    type Error;
    fn decode_add_file<'a>(&mut self, buf: &'a [u8]) -> Result<Vec<&'a Path>, Self::Error>;
    fn decode_opcode(&mut self, u: u8) -> Result<Opcode, Self::Error>;
}
