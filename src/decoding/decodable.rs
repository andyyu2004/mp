use crate::Decoder;

pub trait Decodable<'a>
where
    Self: Sized,
{
    fn decode<D: ?Sized>(buf: &'a [u8], decoder: &mut D) -> Result<Self, D::Error>
    where
        D: Decoder;
}
