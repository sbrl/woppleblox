use std::collections::HashMap;
use std::path::Path;

use fluent::{ FluentResource, FluentArgs };
use fluent::concurrent::{ FluentBundle };
use unic_langid::{ LanguageIdentifier, langid };
use unic_langid::LanguageIdentifierError;

mod embedded_files;

impl Clone for Translations {
	fn clone(&self) -> Self {
		Translations::default()
	}
}

// Fluent example: https://github.com/projectfluent/fluent-rs/blob/master/fluent-bundle/examples/simple-app.rs

pub struct Translations {
	lang_default: LanguageIdentifier,
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
			lang_default: langid!("en_GB"),
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
	
	fn get_languages(&self) -> Vec<LanguageIdentifier> {
		let mut result = Vec::new();
		for key in self.langs.keys() {
			result.push(key.clone());
		}
		result
	}
	
	fn get_languages_str(&self) -> Vec<String> {
		let list = self.get_languages();
		let mut result = Vec::new();
		for item in list {
			result.push(item.language().to_owned());
		}
		result
	}
	
	fn negotiate_lang(&self, wanted: &str) -> Vec<LanguageIdentifier> {
		let our_langs: Vec<String> = self.get_languages_str();
		let our_langs_ref: Vec<&str> = our_langs.iter().map(AsRef::as_ref).collect();
		
		let intersect = accept_language::intersection(wanted, our_langs_ref);
		let mut result = Vec::new();
		for str in intersect {
			result.push(str.parse().expect(&format!("Failed to parse language code {} when negotiating languages", str)));
		}
		result.push(self.lang_default.to_owned());
		result
	}
	
	/**
	 * Given a translation code, some args, and the Accept-Language header of a request,
	 * returns a translated string with the ggiven arguments substituted in.
	 *
	 * @param	wanted				The accept-language header value from the client's request.
	 * @param	translation_code	The identifier of the string in the fluent translation file to substitute the arguments into and return.
	 * @param	args				The FluentArgs to substitute in.
	 */
	pub fn translate(&self, wanted: &str, translation_code: &str, args: FluentArgs) -> Result<String, String> {
		let langs_wanted = self.negotiate_lang(wanted);
		for lang in langs_wanted {
			if !self.langs.contains_key(&lang) {
				continue;
			}
			
			let bundle = self.langs.get(&lang).expect(&format!("Failed to find FluentBundle for language code {} - even though we checked and the HashMap claimed it exists O.o", lang));
			let msg = match bundle.get_message(translation_code) {
				Some(msg_inner) => msg_inner,
				None => continue
			};
			let pattern = match msg.value {
				Some(pattern_inner) => pattern_inner,
				None => continue
			};
			let mut errors = vec![];
			let result = bundle.format_pattern(&pattern, Some(&args), &mut errors).to_string();
			if errors.len() > 0 {
				error!("Errors encountered formatting fluent pattern with code {}:", translation_code);
				for error in errors {
					error!("{:#?}", error);
				}
			}
			return Ok(result);
		}
		
		Err(format!("Error: Failed to fetch translation for code {} (does it exist in the fluent translation file?)", translation_code))
	}
	
	/**
	 * Convenient alternative to .translate() with fewer arguments.
	 * @param	wanted				The accept-language header value.
	 * @param	translation_code	The translation code string to fetch and translate.
	 */
	pub fn translate_simple(&self, wanted: &str, translation_code: &str) -> Result<String, String> {
		self.translate(wanted, translation_code, FluentArgs::default())
	}
}
