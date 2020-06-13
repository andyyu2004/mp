#![feature(type_alias_impl_trait)]
#![feature(stmt_expr_attributes)]

mod cli;
mod client;
mod connection;
mod error;
mod protocol;
mod ui;

use client::*;
use connection::Connection;
use error::*;
use log::LevelFilter;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use ui::*;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> ClientResult<()> {
    let matches = cli::get_args();

    let mut connection = Connection::new("/tmp/mp-client")?;

    if let Some(matches) = matches.subcommand_matches("add") {
        let files: Vec<&str> = matches.values_of("FILES").unwrap().collect();
        connection
            .add_files(files.into_iter().map(Path::new).collect())
            .await?;
    } else {
        // if no arguments were provided, start the ui
        simple_logging::log_to_file("log.log", LevelFilter::Trace)?;
        let mut client = Client::new(&mut connection);
        client.init().await?;
        let client = Arc::new(Mutex::new(client));
        let mut ui = UI::new(Arc::clone(&client));
        ui.start().await?;
    }

    Ok(())
}
