use crate::Decoder;

/// structures that are decodable
pub trait Decode<'a>
where
    Self: Sized,
{
    fn decode<D: ?Sized>(buf: &'a [u8], decoder: &mut D) -> Result<Self, D::Error>
    where
        D: Decoder;
}
