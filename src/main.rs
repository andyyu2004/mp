mod db;
mod error;
mod file;
mod media;
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
use media::MPState;
use media::{MediaEventHandler, Player};

use server::Server;
use std::io;
use std::sync::{mpsc, Arc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = "/tmp/mp-server";
    let mut db = Database::new();

    let (tx, rx) = mpsc::channel();
    let mpstate = Arc::new(Mutex::new(MPState::default()));
    let player = Player::new(tx, Arc::clone(&mpstate));

    let mut vlc_event_handler = MediaEventHandler::new(mpstate, rx);
    std::thread::spawn(move || vlc_event_handler.listen());

    let mut server = Server::new(path, &mut db, player)?;
    server.start().await;
    Ok(())
}
