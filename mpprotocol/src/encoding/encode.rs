use crate::Encoder;

/// a structure that can be encoded
pub trait Encode {
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder;
}
