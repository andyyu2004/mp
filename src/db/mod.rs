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
use entry::*;

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

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
