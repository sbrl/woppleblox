
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
    // Whether we should write a config file if it doesn't exist already or not.
    #[serde(skip)]
    pub config_write : bool,
    
    pub http : SettingsHttp
}
impl Default for Settings {
    fn default() -> Settings {
        Settings {
            config_write: false,
            http: SettingsHttp::default()
        }
    }
}
