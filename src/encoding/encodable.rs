use crate::{Encoder, Request};
use std::path::Path;

pub trait Encodable {
    fn encode<E>(&self, buf: &mut [u8], encoder: E) -> Result<usize, E::Error>
    where
        E: Encoder;
}

impl<'r> Encodable for Request<'r> {
    fn encode<E>(&self, buf: &mut [u8], encoder: E) -> Result<usize, E::Error>
    where
        E: Encoder,
    {
        match self {
            Self::AddFile(paths) => paths.encode(buf, encoder),
        }
    }
}

impl Encodable for Vec<&Path> {
    fn encode<E>(&self, buf: &mut [u8], encoder: E) -> Result<usize, E::Error>
    where
        E: Encoder,
    {
        encoder.encode_add_file(self, buf)
    }
}
