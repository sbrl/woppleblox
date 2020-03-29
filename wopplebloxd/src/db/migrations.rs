use super::definitions::{ Connection };
// use rusqlite::Connection;

mod embedded {
    use refinery::embed_migrations;
    
    embed_migrations!("wopplebloxd/migrations");
}

pub struct SqliteMigrator;

impl Default for SqliteMigrator {
    fn default() -> SqliteMigrator {
        SqliteMigrator {
            
        }
    }
}

impl SqliteMigrator {
    pub fn migrate(&self, connection : &mut Connection) {
        warn!("Warning: We haven't implemented database migrations yet");
        // embedded::migrations::runner().run(&mut connection);
    }
}
