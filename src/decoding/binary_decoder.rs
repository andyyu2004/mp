use crate::*;
use std::path::Path;

pub struct BinaryDecoder;

impl Decoder for BinaryDecoder {
    type Error = ProtocolError;
    fn decode_add_file<'a>(&mut self, buf: &'a [u8]) -> Result<Vec<&'a Path>, Self::Error> {
        let opcode = Opcode::from_u8(buf[0])?;
        todo!()
    }

    fn decode_opcode(&mut self, u: u8) -> Result<Opcode, Self::Error> {
        Opcode::from_u8(u)
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::*;

    #[test]
    fn parse_add_file() -> crate::ProtocolResult<()> {
        let paths = vec!["/music/a", "music/b"];
        let mut buf = [0u8; 1024];
        let count = gen::add_file(&paths, &mut buf)?;
        let rd = parse::parse_request(&buf[..count])?;
        // assert_eq!(rd, Request::AddFile(paths));

        Ok(())
    }
}
