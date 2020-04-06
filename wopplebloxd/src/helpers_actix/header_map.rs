use actix_web::http::header::{ HeaderMap, HeaderValue };

/**
 * Helper trait to make working with HTTP headers less painful
 */
pub trait HeaderMapHelper {
	/**
	 * Fetch the first value associated with the given HTTP header name.
	 * If it doesn't exist, return default_or instead.
	 * @param	name		The name of the HTTP header to fetch.
	 * @param	default_or	The default value to return if no value was found.
	 */
	fn get_or_default(&self, name: &str, default_or: &str) -> HeaderValue;
}


impl HeaderMapHelper for HeaderMap {
	fn get_or_default(&self, name: &str, default_or: &str) -> HeaderValue {
		match self.get(name) {
			Some(value) => value.clone(),
			None => HeaderValue::from_str(default_or).expect("Error: Failed to convert default value to HeaderValue.")
		}
	}
}
