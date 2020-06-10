use std::io;
use std::path::Path;
use tokio::net::UnixDatagram;

pub(crate) struct Connection {
    socket: UnixDatagram,
}

impl Connection {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let socket = UnixDatagram::bind(path)?;
        socket.connect("/tmp/mp-server")?;
        Ok(Self { socket })
    }

    pub async fn send(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.socket.send(bytes).await?;
        Ok(())
    }

    pub async fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf).await
    }
}

// -fn handle_streams(listener: UnixListener) -> io::Result<()> {
//     -    for stream in listener.incoming() {
//         -        handle_stream(stream?)?;
//         -    }
//     -    Ok(())
//         -}
// -
//     -fn handle_stream(mut stream: UnixStream) -> io::Result<()> {
//         -    let mut buffer = [0; 128];
//         -    let count = stream.read(&mut buffer)?;
//         -    println!("stream recv {:?}", &buffer[..count]);
//         -    Ok(())
//             -}
