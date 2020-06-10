use crate::{Encoder, Encoding, Opcode, ProtocolError, ProtocolResult};
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct BinaryEncoder<W> {
    pub writer: W,
    count: usize,
}

impl<W> BinaryEncoder<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer, count: 0 }
    }

    fn write_bincode<S>(&mut self, item: &S) -> ProtocolResult<()>
    where
        S: Serialize,
    {
        self.count += bincode::serialized_size(item).unwrap() as usize;
        bincode::serialize_into(&mut self.writer, item).unwrap();
        Ok(())
    }

    pub fn get_byte_count(self) -> usize {
        self.count
    }
}

impl<W> Encoder for &mut BinaryEncoder<W>
where
    W: Write,
{
    type Ok = ();
    type Error = ProtocolError;

    fn encode_add_file(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> ProtocolResult<()> {
        self.count += self
            .writer
            .write(&[Encoding::Binary as u8, Opcode::AddFile as u8])?;
        let absolute_paths_bufs = paths
            .into_iter()
            .map(fs::canonicalize)
            .collect::<Result<Vec<PathBuf>, _>>()?;
        let absolute_paths: Vec<&Path> = absolute_paths_bufs.iter().map(|p| p.as_path()).collect();
        self.write_bincode(&absolute_paths)?;
        Ok(())
    }
}
