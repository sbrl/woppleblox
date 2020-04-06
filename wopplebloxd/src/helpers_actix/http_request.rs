use actix_web::HttpRequest;

use super::header_map::HeaderMapHelper;

/**
 * Helper methods for a HTTP request.
 */
pub trait HttpRequestHelper {
    /**
     * Fetch the requested language(s) from the request.
     * Falls back to en-GB if none were specified or the header value was invalid.
     */
    fn get_req_lang(&self) -> String;
}

impl HttpRequestHelper for HttpRequest {
    fn get_req_lang(&self) -> String {
        match self.headers().get_or_default("accept-language", "en-GB").to_str() {
            Ok(result) => result,
            Err(msg) => {
                warn!("Warning: Invalid accept-language header value detected, falling back to default (message: {})", msg);
                "en-GB"
            }
        }.to_string()
    }
}
