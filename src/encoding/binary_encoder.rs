use crate::{Encoder, Encoding, Opcode, ProtocolError, ProtocolResult};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

pub struct BinaryEncoder;

impl BinaryEncoder {
    /// merge the serialized_size and serialize_into as one operation
    fn encode<T>(buf: &mut [u8], item: &T) -> usize
    where
        T: Serialize,
    {
        bincode::serialize_into(buf, &item).unwrap();
        bincode::serialized_size(item).unwrap() as usize
    }
}

impl Encoder for BinaryEncoder {
    type Error = ProtocolError;

    fn encode_add_file(
        &self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        buf: &mut [u8],
    ) -> ProtocolResult<usize> {
        buf[0] = Encoding::Binary as u8;
        buf[1] = Opcode::AddFile as u8;
        let absolute_paths_bufs = paths
            .into_iter()
            .map(fs::canonicalize)
            .collect::<Result<Vec<PathBuf>, _>>()?;
        let absolute_paths: Vec<&Path> = absolute_paths_bufs.iter().map(|p| p.as_path()).collect();
        let n = Self::encode(&mut buf[2..], &absolute_paths);
        Ok(2 + n)
    }
}
