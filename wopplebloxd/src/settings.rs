use std::fs;
use std::path::Path;

// #[macro_use] // for toml::toml!, which are aren't currently using
use toml;

mod definitions;
pub use definitions::Settings;

impl Settings {
    pub fn new() -> Settings {
        Settings::default()
    }
    
    pub fn load_settings_file(&mut self, filename : String) {
        let config_str = match fs::read_to_string(&filename) {
            Ok(str) => str,
            Err(_error) => {
                // TODO: Check if this is an std::io::ErrorKind::NotFound
                
                if !Path::new("config.toml").exists() {
                    match fs::write(
                        "config.toml",
                        toml::to_string_pretty(&self).expect("Error: Failed to serialise default settings O.o (this is a bug, please get in touch)")
                    ) {
                        Ok(_) => (),
                        Err(error) => {
                            warn!("Warning: Didn't find a config file, but failed to write a default one to disk.");
                            warn!("Details: {}", error);
                        }
                    }
                }
                // println!("{}", error);
                
                // Zero-length string = use default settings
                "".to_string()
            }
        };
        
        // If it's of length zero, then jsut use the default settings that are already loaded
        if config_str.len() == 0 { return }
        
        let config_obj = toml::from_str(&config_str).unwrap();
        info!("{:?}", config_obj);
        
        self.settings = config_obj;
    }
    
}
