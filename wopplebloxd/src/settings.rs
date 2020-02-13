use std::fs;

// #[macro_use] // for toml::toml!, which are aren't currently using
use toml;

mod definitions;
use definitions::{ Settings };

pub struct SettingsManager {
    settings : Settings
}

impl SettingsManager {
    pub fn new() -> SettingsManager {
        SettingsManager {
            settings : Settings::default()
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
