mod album;
mod artist;
mod entry;
mod mpdb;
mod schema;
mod track;

use diesel::prelude::*;
use dotenv::dotenv;

pub use album::*;
pub use artist::*;
pub use entry::*;

pub use track::*;

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn new() -> Self {
        let connection = Self::establish_connection();
        Self { connection }
    }

    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").ok().unwrap_or_else(|| {
            let mut path = dirs::home_dir().unwrap();
            path.push(".config/mpserver/db.sqlite");
            path.to_str().unwrap().to_owned()
        });
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|err| panic!("error connecting to {} {:?}", database_url, err))
    }
}
