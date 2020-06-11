mod db;
mod error;
mod files;
mod mp_server;
mod server;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

// mpris
// mpris-listen

use db::Database;
use error::*;

use server::Server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = "/tmp/mp-server";
    let mut db = Database::new();
    let mut server = Server::new(path, &mut db)?;
    server.start().await;
    Ok(())
}
