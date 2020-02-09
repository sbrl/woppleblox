use std::io;
use std::fs;

use toml;

mod definitions;
use definitions::{ Settings };

struct SettingsManager {
    settings : Settings
}

impl SettingsManager {
    pub fn new() -> SettingsManager {
        SettingsManager {
            settings: Settings {  }
        }
    }
    
    pub fn load_settings_file(filename : String) {
        let config = fs::read_to_string(filename).expect(
            &format!("Error: Failed to read config file from {}.", filename)
        );
        
        
    }
}
