#![feature(async_closure)]

mod cli;
mod client;
mod cmd;
mod error;
mod network;
mod ui;
mod util;

use client::*;
use error::*;
use log::LevelFilter;
use mp_protocol::ProtocolResult;
use network::Connection;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use ui::*;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> ClientResult<()> {
    let matches = cli::get_args();

    let (tx, rx) = mpsc::channel();
    let client = Arc::new(Mutex::new(Client::new()));
    let mut connection =
        Connection::new("/tmp/mp-server", Arc::clone(&client), tx.clone(), rx).await?;

    if let Some(matches) = matches.subcommand_matches("add") {
        let files: Vec<&str> = matches.values_of("FILES").unwrap().collect();
        connection.dispatch_add_files(files.into_iter().map(Path::new).collect()).await?;
    } else if let Some(matches) = matches.subcommand_matches("play") {
        if matches.is_present("next") {
            connection.dispatch_play_next().await?;
        } else if matches.is_present("prev") {
            connection.dispatch_play_prev().await?;
        } else {
            match matches.values_of("TRACK") {
                Some(mut track) => {
                    let _track = track.next();
                    // do some fuzzy search or something on this input and play the closest match
                }
                None => connection.dispatch_play().await?,
            }
        }
    } else if let Some(_) = matches.subcommand_matches("pause") {
        connection.dispatch_pause().await?;
    } else if let Some(matches) = matches.subcommand_matches("canonicalize") {
        let src = matches.value_of("SRC").unwrap();
        let dst = matches.value_of("dest").unwrap();
        connection.dispatch_canonicalize(src, dst).await?;
    } else {
        // if no arguments were provided, start the ui
        #[cfg(debug_assertions)]
        simple_logging::log_to_file("log.log", LevelFilter::Trace)?;
        let io_handle = std::thread::spawn(move || io_main(connection));
        let mut ui = UI::new(Arc::clone(&client), tx);
        ui.start()?;
        io_handle.join().unwrap()?;
        // UI::start will close the connection on completion so we shouldn't close it again
        return Ok(());
    }

    connection.close().await?;
    Ok(())
}

#[tokio::main]
async fn io_main(mut connection: Connection) -> ProtocolResult<()> {
    connection.listen().await
}
