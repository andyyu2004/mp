use std::io;
use std::io::prelude::*;
use std::os::unix::net::{UnixDatagram, UnixStream};

// mpris
// mpris-listen

fn main() -> io::Result<()> {
    let path = "/tmp/mp-server";
    let socket = UnixDatagram::bind(path)?;

    loop {
        let mut buf = [0; 100];
        let (count, addr) = socket.recv_from(&mut buf)?;

        let addr = addr.as_pathname().unwrap();
        println!("server recv {:?} from {}", &buf[..count], addr.display());

        socket.send_to(b"received", addr)?;

        let mut stream = UnixStream::connect(format!("{}-stream", addr.display()))?;

        println!("socket {:?} sent {:?}", addr, &buf[..count]);

        for i in 0..5 {
            stream.write_all(&[i])?;
        }
    }

    // std::fs::remove_file(path)
}
