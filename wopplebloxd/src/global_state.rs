use crate::db::{Database};
use crate::settings::Settings;

#[derive(Clone)]
pub struct GlobalState {
    pub sitename : String,
    pub db : Database
}

impl GlobalState {
    // pub fn new() -> GlobalState {
    //     GlobalState::default()
    // }
    pub fn new(settings : Settings) -> GlobalState {
        return GlobalState {
            sitename: "Woppleblox".to_string(),
            db : Database::new(settings.db.filename)
        }
    }
}
