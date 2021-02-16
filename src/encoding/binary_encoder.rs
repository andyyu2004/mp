use crate::{Encoder, Encoding, Opcode, ProtocolError};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct BinaryEncoder<W> {
    pub writer: W,
}

impl<W> BinaryEncoder<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Encoder for &mut BinaryEncoder<W>
where
    W: Write,
{
    type Error = ProtocolError;
    type Ok = ();

    fn encode_opcode(&mut self, opcode: Opcode) -> Result<Self::Ok, Self::Error> {
        self.writer.write(&[Encoding::Binary as u8, opcode as u8])?;
        Ok(())
    }

    fn encode_path(&mut self, path: &Path) -> Result<Self::Ok, Self::Error> {
        Ok(bincode::serialize_into(&mut self.writer, &path).unwrap())
    }

    fn encode_paths(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_opcode(Opcode::AddFile)?;
        let absolute_paths_bufs =
            paths.into_iter().map(fs::canonicalize).collect::<Result<Vec<PathBuf>, _>>()?;
        let absolute_paths: Vec<&Path> = absolute_paths_bufs.iter().map(|p| p.as_path()).collect();
        bincode::serialize_into(&mut self.writer, &absolute_paths).unwrap();
        Ok(())
    }

    fn encode_i32(&mut self, i: i32) -> Result<Self::Ok, Self::Error> {
        self.writer.write(&i32::to_be_bytes(i))?;
        Ok(())
    }

    fn encode_i64(&mut self, i: i64) -> Result<Self::Ok, Self::Error> {
        self.writer.write(&i64::to_be_bytes(i))?;
        Ok(())
    }
}
