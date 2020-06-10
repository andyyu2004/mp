use crate::Decoder;

/// structures that are decodable

pub trait Decode<'a>
where
    Self: Sized,
{
    fn decode<D>(buf: &'a [u8], decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder;
}
