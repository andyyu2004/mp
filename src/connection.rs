use async_std::os::unix::net::UnixDatagram;
use async_std::path::Path;
use std::io;

pub(crate) struct Connection {
    socket: UnixDatagram,
}

impl Connection {
    pub async fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let socket = UnixDatagram::bind(path).await?;
        socket.connect("/tmp/mp-server").await?;
        Ok(Self { socket })
    }

    pub async fn send(&self, bytes: &[u8]) -> io::Result<()> {
        self.socket.send(bytes).await?;
        Ok(())
    }

    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
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
