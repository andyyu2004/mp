use crate::{Encoder, Encoding, Opcode, ProtocolError, ProtocolResult};
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

    fn write_encoding_opcode(&mut self, opcode: Opcode) -> ProtocolResult<()> {
        self.writer.write(&[Encoding::Binary as u8, opcode as u8])?;
        Ok(())
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
        self.write_encoding_opcode(Opcode::AddFile)?;
        let absolute_paths_bufs = paths
            .into_iter()
            .map(fs::canonicalize)
            .collect::<Result<Vec<PathBuf>, _>>()?;
        let absolute_paths: Vec<&Path> = absolute_paths_bufs.iter().map(|p| p.as_path()).collect();
        bincode::serialize_into(&mut self.writer, &absolute_paths).unwrap();
        Ok(())
    }

    fn encode_fetch_tracks(&mut self) -> Result<Self::Ok, Self::Error> {
        self.write_encoding_opcode(Opcode::FetchTracks)
    }

    fn encode_play_track(&mut self, track_id: i32) -> Result<Self::Ok, Self::Error> {
        self.write_encoding_opcode(Opcode::PlayTrack)?;
        self.writer.write(&track_id.to_be_bytes())?;
        Ok(())
    }
}
