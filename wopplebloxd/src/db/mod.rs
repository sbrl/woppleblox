use r2d2_sqlite::{ self, SqliteConnectionManager };
// use rusqlite::{ params, Connection, Result };
// use chrono::DateTime;

mod dataclasses;
mod definitions;
mod migrations;
mod repositories;

// pub use dataclasses::{User, Alias, Post, Attachment};
pub use dataclasses::user::User;
pub use dataclasses::alias::Alias;
pub use dataclasses::post::Post;
pub use dataclasses::attachment::Attachment;

use migrations::SqliteMigrator;
use definitions::{ Pool, Connection };

// pub use User;
// pub use Alias;
// pub use Post;
// pub use Attachment;

#[derive(Clone)]
pub struct Database {
    con : Pool
}

impl Database {
    pub fn new(filename : String) -> Database {
        let result = Database {
            con: Database::make_db_connection(filename).unwrap()
        };
        result.init();
        
        result
    }
    
    fn make_db_connection(filename : String) -> std::result::Result<Pool, String> {
        match Pool::new(SqliteConnectionManager::file(filename)) {
            Ok(pool) => Ok(pool),
            Err(message) => Err(message.to_string())
        }
            // .expect("Error: Failed to open connection to the SQLite database (does the parent directory exist, and does wopplebloxd have read+write permissions?)")
    }
    
    pub fn init(&self) {
        let migrator = SqliteMigrator::default();
        migrator.migrate(&mut self.con.get().expect("Error: Failed to get a connection form the pool to the SQLite database to perform database migrations on startup."));
    }
    
    pub fn conn(&mut self) -> PooledConnection {
        self.con.get().unwrap()
    }
}
