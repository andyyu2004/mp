use crate::files;
use mp_protocol::{ProtocolResult, BUF_CAP};
use mp_protocol::{Request, Response};
use std::convert::TryFrom;
use std::io;
use tokio::net::UnixDatagram;

pub(crate) struct Server {
    socket: UnixDatagram,
}

impl Server {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            socket: UnixDatagram::bind(path)?,
        })
    }

    async fn listen(&mut self) -> ProtocolResult<()> {
        let mut buf = [0; BUF_CAP];
        let (count, addr) = self.socket.recv_from(&mut buf).await?;
        println!("recv");

        let addr = addr.as_pathname().unwrap();

        let req = Request::try_from(&buf[..count])?;

        let res = self.handle_request(req).await?;
        dbg!(res);

        self.socket.send_to(b"00", addr).await?;

        Ok(())
    }

    pub async fn handle_request(&mut self, req: Request<'_>) -> ProtocolResult<Response> {
        match req {
            Request::AddFile(paths) => files::add_files(&paths)?,
        };

        Ok(Response)
    }

    pub async fn start(&mut self) -> ! {
        loop {
            if let Err(err) = self.listen().await {
                println!("err: {}", err)
            }
        }

        // std::fs::remove_file(path)
    }
}
