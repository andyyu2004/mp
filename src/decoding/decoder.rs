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

    fn decode_i32(&mut self, buf: &[u8]) -> Result<i32, Self::Error> {
        D::decode_i32(self, buf)
    }

    fn decode_i64(&mut self, buf: &[u8]) -> Result<i64, Self::Error> {
        D::decode_i64(self, buf)
    }
}

pub trait Decoder {
    type Error;
    fn decode_add_file<'a>(&mut self, buf: &'a [u8]) -> Result<Vec<&'a Path>, Self::Error>;
    fn decode_opcode(&mut self, u: u8) -> Result<Opcode, Self::Error>;
    fn decode_i32(&mut self, buf: &[u8]) -> Result<i32, Self::Error>;
    fn decode_i64(&mut self, buf: &[u8]) -> Result<i64, Self::Error>;
}
