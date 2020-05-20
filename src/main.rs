use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::unix::net::{UnixDatagram, UnixListener, UnixStream};
use std::thread;

fn main() -> io::Result<()> {
    let path = "/tmp/mpclient";
    let socket = UnixDatagram::bind(path)?;
    socket.connect("/tmp/mp-server")?;

    let stream_path = format!("{}-stream", path);
    let listener = UnixListener::bind(stream_path)?;
    let stream_handler = thread::spawn(|| handle_streams(listener));

    loop {
        println!("waiting for input...");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        socket.send(b"yeet")?;
        let mut buf = [0; 100];
        let count = socket.recv(&mut buf)?;
        println!("client recv {:?} from server", &buf[..count]);
    }

    stream_handler.join().unwrap()?;

    Ok(())
}

fn handle_streams(listener: UnixListener) -> io::Result<()> {
    for stream in listener.incoming() {
        handle_stream(stream?)?;
    }
    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> io::Result<()> {
    let mut buffer = [0; 128];
    let count = stream.read(&mut buffer)?;
    println!("stream recv {:?}", &buffer[..count]);
    Ok(())
}
