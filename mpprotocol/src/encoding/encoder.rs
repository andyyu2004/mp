use crate::Opcode;
use std::path::Path;

pub trait Encoder {
    type Ok;
    type Error;

    fn encode_opcode(&mut self, opcode: Opcode) -> Result<Self::Ok, Self::Error>;
    fn encode_i32(&mut self, i: i32) -> Result<Self::Ok, Self::Error>;
    fn encode_i64(&mut self, i: i64) -> Result<Self::Ok, Self::Error>;
    fn encode_path(&mut self, path: &Path) -> Result<Self::Ok, Self::Error>;
    fn encode_paths(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self::Ok, Self::Error>;

    // convenience functions
    fn encode_op_files(
        &mut self,
        opcode: Opcode,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_opcode(opcode)?;
        self.encode_paths(paths)
    }
    fn encode_op_i32(&mut self, opcode: Opcode, i: i32) -> Result<Self::Ok, Self::Error> {
        self.encode_opcode(opcode)?;
        self.encode_i32(i)
    }

    fn encode_op_i64(&mut self, opcode: Opcode, i: i64) -> Result<Self::Ok, Self::Error> {
        self.encode_opcode(opcode)?;
        self.encode_i64(i)
    }
}
