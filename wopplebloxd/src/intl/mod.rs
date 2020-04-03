use std::collections::HashMap;
use std::path::Path;

use fluent::{ FluentResource };
use fluent::concurrent::{ FluentBundle };
use unic_langid::{ LanguageIdentifier };

mod embedded_files;

impl Clone for Translations {
    fn clone(&self) -> Self {
        Translations::default()
    }
}

// Fluent example: https://github.com/projectfluent/fluent-rs/blob/master/fluent-bundle/examples/simple-app.rs

pub struct Translations {
    langs: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>
}

impl Default for Translations {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Translations {
    pub fn new(quiet: bool) -> Translations {
        let mut result = Translations {
            langs: HashMap::default()
        };
        result.populate_map(quiet);
        result
    }
    
    fn populate_map(&mut self, quiet: bool) {
        // We have a quiet argument here because when we clone we don't want to spam log messages
        for filename_raw in embedded_files::EmbeddedFiles::iter() {
            let filename = filename_raw.to_string();
            let lang_code_string = Path::new(&filename).file_stem().unwrap().to_str().unwrap();
            let lang_code: LanguageIdentifier = lang_code_string.parse()
                .expect("Error: Failed to parse language identifier from embedded translations list");
            
            if !quiet { debug!("Found {} → {}", filename, lang_code); }
            
            let resource = FluentResource::try_new(embedded_files::get_embedded_file(&filename))
                .expect(&format!("Error: Failed to parse fluent translation file for {}", lang_code));
            
            let mut bundle = FluentBundle::<FluentResource>::new(&[lang_code.clone()]);
            bundle.add_resource(resource)
                .expect(&format!("Error: Failed to parse fluent translations file for lanaguage {}.", lang_code));
            
            self.langs.insert(lang_code, bundle);
        }
    }
    
    
}
