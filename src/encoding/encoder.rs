use crate::Opcode;
use std::path::Path;

pub trait Encoder {
    type Ok;
    type Error;

    fn encode_opcode(&mut self, opcode: Opcode) -> Result<Self::Ok, Self::Error>;
    /// some operation on track_id
    fn encode_f_track(&mut self, opcode: Opcode, track_id: i32) -> Result<Self::Ok, Self::Error>;
    fn encode_seek(&mut self, seek_amount: i64) -> Result<Self::Ok, Self::Error>;
    fn encode_add_file(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self::Ok, Self::Error>;
}
