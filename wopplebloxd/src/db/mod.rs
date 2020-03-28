use r2d2_sqlite::{ self, SqliteConnectionManager };
// use rusqlite::{ params, Connection, Result };
// use chrono::DateTime;

mod dataclasses;

// pub use dataclasses::{User, Alias, Post, Attachment};
pub use dataclasses::user::User;
pub use dataclasses::alias::Alias;
pub use dataclasses::post::Post;
pub use dataclasses::attachment::Attachment;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

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
        
    }
}
