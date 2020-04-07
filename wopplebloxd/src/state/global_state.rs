use crate::db::{ Database, repositories::UserRepository };
use crate::settings::Settings;
use crate::intl::Translations;
use crate::db::{ Connection };

// #[derive(Clone)]
pub struct GlobalState {
    pub sitename: String,
    pub settings: Settings,
    pub db: Database,
    pub tr: Translations,
    pub env: GlobalEnvironment
}

impl GlobalState {
    // pub fn new() -> GlobalState {
    //     GlobalState::default()
    // }
    pub fn new(settings : Settings) -> GlobalState {
        let mut db = Database::new(settings.db.filename.clone());
        let env = GlobalEnvironment::new(db.conn());
        
        GlobalState {
            sitename: "Woppleblox".to_string(),
            settings: settings.clone(),
            db : db,
            tr: Translations::new(false),
            env: env
        }
    }
}

pub struct GlobalEnvironment {
    pub firstrun_complete: bool
}

impl GlobalEnvironment {
    pub fn new(conn: Connection) -> GlobalEnvironment {
        GlobalEnvironment {
            // The firstrun wizard is complete if we have more than 1 user in the system
            firstrun_complete: UserRepository::has_users(conn).unwrap()
        }
    }
}
