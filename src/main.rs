mod data;
mod files;
mod server;

// mpris
// mpris-listen

use server::Server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = "/tmp/mp-server";
    let mut server = Server::new(path)?;
    server.start().await;
    Ok(())
}
