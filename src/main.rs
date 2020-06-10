mod cli;
mod connection;
mod protocol;

use async_std::path::Path;
use connection::Connection;
use mp_protocol::ProtocolResult;

#[async_std::main]
async fn main() -> ProtocolResult<()> {
    let matches = cli::get_args();

    let mut connection = Connection::new("/tmp/mp-client").await?;

    if let Some(matches) = matches.subcommand_matches("add") {
        let files: Vec<&str> = matches.values_of("FILES").unwrap().collect();
        connection
            .add_files(files.into_iter().map(Path::new).collect())
            .await?;
    }

    Ok(())
}
