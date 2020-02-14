use actix_web::{ HttpRequest, HttpResponse };

#[derive(RustEmbed)]
#[folder = "wopplebloxd/static"]
struct StaticFiles;

fn handle_embedded_file(path: &str) -> HttpResponse {
    // This is fine, because there aren't any private embedded resources in the above
    // If we did have any such resources, we could always use a second struct to store them separately
    match StaticFiles::get(path) {
        Some(content) => {
            let body = 
            
            // Omitting the last comma here means that the last statement is the return value
        }
        None => HttpResponse::NotFound().body(format!(
            "Oops, the file at {} couldn't be found (is there a typo in the url?)",
            path
        ))
    }
}

pub fn handle_static(request: HttpRequest) -> HttpResponse {
    handle_embedded_file(&request.path()["/static/".len()..])
}
