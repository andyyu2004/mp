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
use media::Player;
use mp_protocol::{Request, FIN_BYTES};
use rayon::ThreadPoolBuilder;
use server::Server;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> ServerResult<()> {
    let (media_tx, media_rx) = mpsc::channel(1);
    let (server_tx, server_rx) = mpsc::channel(1);
    let mp_state = Arc::new(tokio::sync::Mutex::new(MPState::default()));
    let mut player = Player::new(media_tx.clone(), media_rx, server_tx, Arc::clone(&mp_state));

    // as player is not Send due to vlc just communicate with it using mpsc
    // we execute the server on another thread as the player is not send
    let server = Server::new(media_tx, server_rx, mp_state)?;
    let server = Arc::new(tokio::sync::Mutex::new(server));
    std::thread::spawn(move || client_listen(server));

    player.listen().await;

    Ok(())
}

/// listen for incoming clients
#[tokio::main]
async fn client_listen(server: Arc<tokio::sync::Mutex<Server>>) -> ServerResult<()> {
    let mut listener = UnixListener::bind("/tmp/mp-server")?;
    let mut incoming = listener.incoming();
    let pool = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    while let Some(client) = incoming.next().await {
        let client = client?;
        let server = Arc::clone(&server);
        pool.spawn(|| handle_client(client, server));
    }
    Ok(())
}

#[tokio::main]
async fn handle_client(client: UnixStream, server: Arc<tokio::sync::Mutex<Server>>) {
    if let Err(err) = handle_client_result(client, server).await {
        println!("{}", err);
    }
}

async fn handle_client_result(
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
