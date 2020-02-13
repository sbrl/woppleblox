
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub http : SettingsHttp
}
impl Default for Settings {
    fn default() ->  Settings {
        Settings {
            http: SettingsHttp::default()
        }
    }
}
