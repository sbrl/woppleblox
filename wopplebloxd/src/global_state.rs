use crate::db::{Database};
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
        return GlobalState {
            sitename: "Woppleblox".to_string(),
            settings: settings.clone(),
            db : Database::new(settings.db.filename),
            tr: Translations::new(false)
        }
    }
}

pub struct GlobalEnvironment {
    pub firstrun_complete: bool
}

impl GlobalEnvironment {
    pub fn new(db: Database) -> GlobalEnvironment {
        GlobalEnvironment {
            firstrun_complete: // TODO: Use db.conn() and a repo to determine whether there are any users in the system yet
        }
    }
}
