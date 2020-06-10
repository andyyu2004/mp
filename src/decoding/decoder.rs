use crate::Opcode;
use std::path::Path;

impl<'d, D: ?Sized> Decoder for &'d mut D
where
    D: Decoder,
{
    type Error = D::Error;
    fn decode_add_file<'a>(&mut self, buf: &'a [u8]) -> Result<Vec<&'a Path>, Self::Error> {
        (**self).decode_add_file(buf)
    }

    fn decode_opcode(&mut self, u: u8) -> Result<Opcode, Self::Error> {
        D::decode_opcode(self, u)
    }
}

pub trait Decoder {
    type Error;
    fn decode_add_file<'a>(&mut self, buf: &'a [u8]) -> Result<Vec<&'a Path>, Self::Error>;
    fn decode_opcode(&mut self, u: u8) -> Result<Opcode, Self::Error>;
}
