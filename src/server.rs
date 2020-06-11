use crate::Database;
use crate::ServerResult;
use mp_protocol::{Request, Response, BUF_CAP};
use std::convert::TryFrom;
use std::io;
use tokio::net::UnixDatagram;

pub(crate) struct Server<'a> {
    socket: UnixDatagram,
    pub(crate) db: &'a mut Database,
}

impl<'a> Server<'a> {
    pub fn new(path: &str, db: &'a mut Database) -> io::Result<Self> {
        Ok(Self {
            socket: UnixDatagram::bind(path)?,
            db,
        })
    }

    async fn listen(&mut self) -> ServerResult<()> {
        let mut buf = [0; BUF_CAP];
        let (count, addr) = self.socket.recv_from(&mut buf).await?;
        let addr = addr.as_pathname().unwrap();

        let req = Request::try_from(&buf[..count])?;

        let res = self.handle_request(req).await?;
        dbg!(res);

        self.socket.send_to(b"00", addr).await?;

        Ok(())
    }

    pub async fn handle_request(&mut self, req: Request<'_>) -> ServerResult<Response> {
        match req {
            Request::AddFile(paths) => self.add_files(&paths)?,
        };

        Ok(Response)
    }

    pub async fn start(&mut self) -> ! {
        loop {
            if let Err(err) = self.listen().await {
                println!("err: {:?}", err)
            }
        }

        // std::fs::remove_file(path)
    }
}
