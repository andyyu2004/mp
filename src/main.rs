#![allow(unused_import)]
#![allow(dead_code)]

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

use mp_protocol::{Request, FIN_BYTES};
use server::Server;
use std::convert::TryFrom;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> ServerResult<()> {
    let (media_tx, media_rx) = mpsc::channel(1);
    let (server_tx, server_rx) = mpsc::channel(1);
    let mpstate = Arc::new(tokio::sync::Mutex::new(MPState::default()));
    let mut player = Player::new(media_tx.clone(), media_rx, server_tx, Arc::clone(&mpstate));

    // as player is not Send due to vlc just communicate with it using mpsc
    // we execute the server on another thread as the player is not send
    let server = Arc::new(tokio::sync::Mutex::new(Server::new(
        media_tx, server_rx, mpstate,
    )?));
    std::thread::spawn(move || listen(server));

    player.listen().await;

    Ok(())
}

#[tokio::main]
async fn listen(server: Arc<tokio::sync::Mutex<Server>>) -> ServerResult<()> {
    let mut listener = UnixListener::bind("/tmp/mp-server")?;
    let mut incoming = listener.incoming();
    while let Some(client) = incoming.next().await {
        let client = client?;
        let server = Arc::clone(&server);
        std::thread::spawn(|| handle_client(client, server));
    }
    Ok(())
}

#[tokio::main]
async fn handle_client(
    mut client: UnixStream,
    server: Arc<tokio::sync::Mutex<Server>>,
) -> ServerResult<()> {
    loop {
        let msg_len = client.read_u32().await? as usize;
        let mut buf = vec![0u8; msg_len];
        client.read_exact(&mut buf).await?;
        if msg_len == 4 && &buf[0..4] == FIN_BYTES {
            break;
        }
        let req = Request::try_from(&buf[..])?;

        let mut server = server.lock().await;
        let res = server.handle_request(req).await;
        let bytes = serde_json::to_vec(&res).unwrap();
        client.write_u32(bytes.len() as u32).await?;
        client.write_all(&bytes).await?;
    }
    Ok(())
}
