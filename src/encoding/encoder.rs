use std::path::Path;

pub trait Encoder {
    type Error;

    fn encode_add_file(
        &self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        buf: &mut [u8],
    ) -> Result<usize, Self::Error>;
}
