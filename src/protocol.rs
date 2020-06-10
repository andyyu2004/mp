use crate::connection::Connection;
use async_std::path::Path;
use mp_protocol::{BinaryEncoder, Encodable, ProtocolResult, Request, BUF_CAP};

impl Connection {
    pub(crate) async fn add_files(&mut self, files: Vec<&Path>) -> ProtocolResult<()> {
        let mut buf = [0u8; BUF_CAP];
        let req = Request::AddFile(files);
        let count = req.encode(&mut buf, BinaryEncoder)?;
        self.send(&buf[..count]).await?;

        let count = self.recv(&mut buf).await?;
        println!("recv: {:x?}", &buf[..count]);
        Ok(())
    }
}
