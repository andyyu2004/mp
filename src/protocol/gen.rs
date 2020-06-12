use crate::{Encoding, Opcode, ProtocolResult};
use std::fs;
use std::path::{Path, PathBuf};

pub fn add_file(paths: &Vec<&str>, buf: &mut [u8]) -> ProtocolResult<usize> {
    buf[0] = Opcode::AddFile as u8;
    buf[1] = Encoding::Binary as u8;
    let absolute_paths_bufs = paths
        .iter()
        .map(fs::canonicalize)
        .collect::<Result<Vec<PathBuf>, _>>()?;
    let absolute_paths: Vec<&Path> = absolute_paths_bufs.iter().map(|p| p.as_path()).collect();
    let n = bincode::serialized_size(&absolute_paths).unwrap() as usize;
    bincode::serialize_into(&mut buf[2..], &absolute_paths).unwrap();
    Ok(2 + n)
}

// return 
pub fn serialize() {

}