#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate diesel;

pub mod constants;
pub mod decoding;
pub mod encoding;
pub mod error;
pub mod opcode;
pub mod request;
pub mod response;
pub mod transmission;
pub mod util;

pub use constants::*;
pub use decoding::*;
pub use encoding::*;
pub use error::{ProtocolError, ProtocolResult};
pub use opcode::Opcode;
pub use request::*;
pub use response::*;
pub use transmission::*;

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::convert::TryFrom;
    use std::path::Path;

    #[test]
    fn encode_decode_add_file() -> crate::ProtocolResult<()> {
        let paths: Vec<&Path> = vec!["test/a", "test/b"].into_iter().map(Path::new).collect();
        let mut buf = vec![];
        let req = Request::AddFile(paths.clone());
        binary_encode_to_bytes(&req, &mut buf)?;
        let decoded = Request::try_from(&buf[..])?;
        let current_dir = std::env::current_dir()?;
        let absolute_paths: Vec<String> = paths
            .into_iter()
            .map(|path| format!("{}/{}", current_dir.to_str().unwrap(), path.to_str().unwrap()))
            .collect();

        assert_eq!(decoded, Request::AddFile(absolute_paths.iter().map(Path::new).collect()));

        Ok(())
    }

    macro_rules! encode_decode {
        ($req:expr) => {{
            let req = $req;
            let mut buf = vec![];
            binary_encode_to_bytes(&req, &mut buf)?;
            let decoded = Request::try_from(&buf[..])?;
            assert_eq!(decoded, req);
            Ok(())
        }};
    }

    #[test]
    fn encode_decode_fetch_tracks() -> ProtocolResult<()> {
        encode_decode!(Request::FetchTracks)
    }

    #[test]
    fn encode_decode_play_track() -> ProtocolResult<()> {
        encode_decode!(Request::PlayTrack(rand::thread_rng().gen()))
    }
}
