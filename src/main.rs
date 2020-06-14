#![feature(type_alias_impl_trait)]
#![feature(stmt_expr_attributes)]

mod cli;
mod client;
mod error;
mod network;
mod ui;
mod util;

use client::*;
use error::*;
use log::LevelFilter;
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
    let mut connection = Connection::new("/tmp/mp-client", Arc::clone(&client), rx, tx.clone())?;

    if let Some(matches) = matches.subcommand_matches("add") {
        let files: Vec<&str> = matches.values_of("FILES").unwrap().collect();
        connection
            .dispatch_add_files(files.into_iter().map(Path::new).collect())
            .await?;
    } else if let Some(matches) = matches.subcommand_matches("play") {
        match matches.values_of("TRACK") {
            Some(mut track) => {
                let _track = track.next();
                // do some fuzzy search or something on this input and play the closest match
            }
            None => connection.dispatch_play().await?,
        }
    } else if let Some(_) = matches.subcommand_matches("pause") {
        connection.dispatch_pause().await?;
    } else {
        // if no arguments were provided, start the ui
        simple_logging::log_to_file("log.log", LevelFilter::Trace)?;
        std::thread::spawn(move || io_main(connection));
        let mut ui = UI::new(Arc::clone(&client), tx);
        ui.start()?;
    }

    Ok(())
}

#[tokio::main]
async fn io_main(mut connection: Connection) {
    connection.listen().await.unwrap()
}
