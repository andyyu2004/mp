use crate::connection::Connection;
use mp_protocol::constants::BUF_CAP;
use mp_protocol::gen;
use mp_protocol::ProtocolResult;

impl Connection {
    pub(crate) fn add_files(&mut self, files: Vec<&str>) -> ProtocolResult<()> {
        let mut buf = [0u8; BUF_CAP];
        let count = gen::add_file(&files, &mut buf)?;
        self.send(&buf[..count])?;

        let count = self.recv(&mut buf)?;
        println!("recv: {:x?}", &buf[..count]);
        Ok(())
    }
}
