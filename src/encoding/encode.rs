use crate::{Encoder, Request};

/// a structure that can be encoded
pub trait Encode {
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder;
}

impl<'r> Encode for Request<'r> {
    fn encode<E>(&self, mut encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
    {
        match self {
            Self::AddFile(paths) => encoder.encode_add_file(paths),
        }
    }
}
