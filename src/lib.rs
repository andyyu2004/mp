#![feature(box_syntax, box_patterns)]

pub mod constants;
pub mod decoding;
pub mod encoding;
pub mod error;
pub mod opcode;
pub mod request;
pub mod response;

pub use constants::*;
pub use decoding::*;
pub use encoding::*;
pub use error::{ProtocolError, ProtocolResult};
pub use opcode::Opcode;
pub use request::*;
pub use response::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;
    use std::path::Path;

    #[test]
    fn encode_decode_add_file() -> crate::ProtocolResult<()> {
        let paths: Vec<&Path> = vec!["test/a", "test/b"]
            .into_iter()
            .map(Path::new)
            .collect();
        let mut buf = [0u8; 1024];
        let req = Request::AddFile(paths.clone());
        let count = binary_encode_to_bytes(&req, &mut buf[..])?;
        let decoded = Request::try_from(&buf[..count])?;
        let current_dir = std::env::current_dir()?;
        let absolute_paths: Vec<String> = paths
            .into_iter()
            .map(|path| {
                format!(
                    "{}/{}",
                    current_dir.to_str().unwrap(),
                    path.to_str().unwrap()
                )
            })
            .collect();

        assert_eq!(
            decoded,
            Request::AddFile(absolute_paths.iter().map(Path::new).collect())
        );

        Ok(())
    }
}
