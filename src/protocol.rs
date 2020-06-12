use crate::connection::Connection;
use mp_protocol::{ProtocolResult, Request, Response, RES_BUF_CAP};
use std::path::Path;

impl Connection {
    pub(crate) async fn add_files(&mut self, files: Vec<&Path>) -> ProtocolResult<Response> {
        let mut buf = vec![0u8; RES_BUF_CAP];
        let req = Request::AddFile(files);
        let count = mp_protocol::binary_encode_to_bytes(&req, &mut buf[..])?;
        self.send(&buf[..count]).await?;

        let count = self.recv(&mut buf).await?;
        let response = serde_json::from_slice::<Response>(&buf[..count])?;
        dbg!(&response);
        Ok(response)
    }
}
