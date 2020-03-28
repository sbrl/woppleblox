
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsHttp {
    pub bind_address : String,
    pub port : i16,
}
impl Default for SettingsHttp {
    fn default() -> SettingsHttp {
        SettingsHttp {
            bind_address: "127.0.0.1".to_string(),
            port: 3015
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    Sqlite
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsDatabase {
    pub mode : DatabaseType,
    pub filename : String
}
impl Default for SettingsDatabase {
    fn default() -> SettingsDatabase {
        SettingsDatabase {
            mode : DatabaseType::Sqlite,
            filename : "./woppleblox.sqlite".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub http : SettingsHttp,
    pub db : SettingsDatabase
}
impl Default for Settings {
    fn default() -> Settings {
        Settings {
            http: SettingsHttp::default(),
            db : SettingsDatabase::default()
        }
    }
}
