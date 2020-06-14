use super::IOEvent;
use crate::Client;
use mp_protocol::ProtocolResult;
use std::io;
use std::path::Path;
use std::sync::{mpsc::Receiver, Arc, Mutex};
use tokio::net::UnixDatagram;

pub(crate) struct Connection {
    socket: UnixDatagram,
    rx: Receiver<IOEvent>,
    client: Arc<Mutex<Client>>,
}

impl Connection {
    pub fn new(
        path: impl AsRef<Path>,
        client: Arc<Mutex<Client>>,
        rx: Receiver<IOEvent>,
    ) -> io::Result<Self> {
        let socket = UnixDatagram::bind(path)?;
        socket.connect("/tmp/mp-server")?;
        Ok(Self { socket, rx, client })
    }

    pub async fn listen(&mut self) -> ProtocolResult<()> {
        while let Ok(event) = self.rx.recv() {
            self.handle_io_event(event).await?;
        }
        Ok(())
    }

    async fn handle_io_event(&mut self, event: IOEvent) -> ProtocolResult<()> {
        match event {
            IOEvent::InitClient => self.init_client().await,
        }
    }

    pub async fn init_client(&mut self) -> ProtocolResult<()> {
        let tracks = self.fetch_tracks().await?;
        let mut client = self.client.lock().unwrap();
        client.state.tracks = tracks;
        Ok(())
    }

    pub async fn send(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.socket.send(bytes).await?;
        Ok(())
    }

    pub async fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf).await
    }
}
