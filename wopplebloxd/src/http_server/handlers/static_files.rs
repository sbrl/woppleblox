use actix_web::{ body::Body, HttpRequest, HttpResponse };
// use actix_web::body::Body;

use std::borrow::Cow;


#[derive(RustEmbed)]
#[folder = "wopplebloxd/static"]
struct StaticFiles;

fn get_embedded_file(path: &str) -> Option<Body> {
    // This is fine, because there aren't any private embedded resources in the above
    // If we did have any such resources, we could always use a second struct to store them separately
    match StaticFiles::get(path) {
        Some(content) => {
            Some(match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            })
        },
        None => None
    }
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    // This is fine, because there aren't any private embedded resources in the above
    // If we did have any such resources, we could always use a second struct to store them separately
    match get_embedded_file(path) {
        Some(body) => {
            HttpResponse::Ok()
                .header(
                    "content-type",
                    mime_guess::from_path(path).first_or_octet_stream()
                ).body(body)
            // Omitting the last comma here means that the last statement is the return value
        }
        None => HttpResponse::NotFound()
            .header("content-type", "text/plain")
            .body(format!(
                "Oops, the file at {} couldn't be found (is there a typo in the url?)",
                path
            ))
    }
}

pub fn print_embedded_files() {
    for file in StaticFiles::iter() {
        info!("{}", file)
    }
}

pub fn handle_static(request: HttpRequest) -> HttpResponse {
    handle_embedded_file(&request.path()["/static/".len()..])
}
