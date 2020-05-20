use std::io;
use std::os::unix::net::UnixDatagram;

pub(crate) struct Connection {
    socket: UnixDatagram,
}

impl Connection {
    pub fn new(socket: UnixDatagram) -> Self {
        Self { socket }
    }

    pub fn send(&self, bytes: &[u8]) -> io::Result<()> {
        self.socket.send(bytes)?;
        Ok(())
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf)
    }
}
