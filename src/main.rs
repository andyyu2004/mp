use std::io;
use std::io::prelude::*;
use std::os::unix::net::{UnixDatagram, UnixStream};

// mpris
// mpris-listen

fn main() -> io::Result<()> {
    let path = "/tmp/mp.sock";
    let socket = UnixDatagram::bind(path)?;

    loop {
        let mut buf = [0; 100];
        let (count, address) = socket.recv_from(&mut buf)?;
        println!("server recv {:?} from {:?}", &buf[..count], address);

        socket.send_to(b"received", address.as_pathname().unwrap())?;

        let mut stream = UnixStream::connect("/tmp/mpstream.sock")?;
        // println!("socket {:?} sent {:?}", address, &buf[..count]);

        for i in 0..5 {
            stream.write_all(&[i])?;
        }
    }

    // std::fs::remove_file(path)
}
