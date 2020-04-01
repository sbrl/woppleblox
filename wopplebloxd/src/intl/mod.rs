use std::collections::HashMap;
use std::path::Path;

use fluent::{ FluentBundle, FluentResource };
use unic_langid::{ LanguageIdentifier };

mod embedded_files;

// HACK: FluentBundle doesn't #[derive(Clone)], so this does that for it
// Ref https://doc.rust-lang.org/std/clone/trait.Clone.html#how-can-i-implement-clone
impl<FluentResource> Clone for FluentBundle<FluentResource> {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for FluentResource {
    fn clone(&self) -> Self {
        *self
    }
}


#[derive(Clone)]
pub struct Translations {
    langs: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>
}

impl Translations {
    pub fn new() -> Translations {
        let mut result = Translations {
            langs: HashMap::default()
        };
        result.populate_map();
        result
    }
    
    fn populate_map(&mut self) {
        for filename_raw in embedded_files::EmbeddedFiles::iter() {
            let filename = filename_raw.to_string();
            let lang_code_string = Path::new(&filename).file_stem().unwrap().to_str().unwrap();
            let lang_code: LanguageIdentifier = lang_code_string.parse()
                .expect("Error: Failed to parse language identifier from embedded translations list");
            
            info!("Found {} → {}", filename, lang_code);
            
            let resource = FluentResource::try_new(embedded_files::get_embedded_file(&filename))
                .expect(&format!("Error: Failed to parse fluent translation file for {}", lang_code));
            
            let mut bundle = FluentBundle::<FluentResource>::new(&[lang_code]);
            
            self.langs.insert(lang_code, bundle);
        }
    }
}
