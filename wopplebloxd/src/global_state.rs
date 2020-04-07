use crate::db::{ Database, repositories::UserRepository };
use crate::settings::Settings;
use crate::intl::Translations;

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
        let db = Database::new(settings.db.filename);
        
        GlobalState {
            sitename: "Woppleblox".to_string(),
            settings: settings.clone(),
            db : db,
            tr: Translations::new(false),
            env: GlobalEnvironment::new(db)
        }
    }
}

pub struct GlobalEnvironment {
    pub firstrun_complete: bool
}

impl GlobalEnvironment {
    pub fn new(db: Database) -> GlobalEnvironment {
        GlobalEnvironment {
            // The firstrun wizard is complete if we have more than 1 user in the system
            firstrun_complete: UserRepository::has_users(db.conn()).unwrap()
        }
    }
}
