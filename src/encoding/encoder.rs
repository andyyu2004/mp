use std::path::Path;

pub trait Encoder {
    type Ok;
    type Error;

    fn encode_add_file(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self::Ok, Self::Error>;
}
