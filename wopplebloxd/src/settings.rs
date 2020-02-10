use std::fs;

// #[macro_use] // for toml::toml!, which are aren't currently using
use toml;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct SettingsHttp {
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
struct Settings {
    pub http : SettingsHttp
}
impl Default for Settings {
    fn default() ->  Settings {
        Settings {
            http: SettingsHttp::default()
        }
    }
}



pub struct SettingsManager {
    settings : Settings
}

impl SettingsManager {
    pub fn new() -> SettingsManager {
        SettingsManager {
            settings : toml::de::from_str(r#"
[http]
bind_address = "127.0.0.1"
port = 3010
"#).expect("Error: Failed to parse default settings definition")
        }
    }
    
    pub fn load_settings_file(&mut self, filename : String) {
        let config_str = fs::read_to_string(&filename).expect(
            &format!("Error: Failed to read config file from {}.", filename)
        );
        let config_obj = toml::from_str(&config_str).unwrap();
        info!("{:?}", config_obj);
        
        self.settings = config_obj;
    }
    
}
