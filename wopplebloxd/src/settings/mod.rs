use std::fs;

// for toml::toml!, which are aren't currently using
// #[macro_use]
// use toml;


mod definitions;
pub use definitions::Settings;

// Adapted from https://stackoverflow.com/a/47142105/1460422 for TOML, but never actually used
// use toml::Value;
// 
// fn merge(a: &mut Value, b: Value) {
//     match (a, b) {
//         (a @ &mut Value::Table(_), Value::Table(b)) => {
//             let a = a.as_table_mut().unwrap();
//             for (k, v) in b {
//                 // This is ok because it the bool false should always be overwritten with v
//                 merge(a.entry(k).or_insert(Value::Boolean(false)), v);
//             }
//         }
//         (a, b) => *a = b,
//     }
// }

impl Settings {
    pub fn new() -> Settings {
        Settings::default()
    }
    
    /// Creates a new Settings object from the contents of a given (TOML) file.
    pub fn from_file(filename : String) -> Settings {
        let config_str = match fs::read_to_string(&filename) {
            Ok(str) => str,
            Err(error) => {
                // println!("{}", error);
                panic!(error); // Oops, something went wrong while reading the config file
            }
        };
        
        let config_obj : Settings = toml::from_str(&config_str).unwrap();
        // info!("{:?}", config_obj);
        return config_obj;
    }
    
}
