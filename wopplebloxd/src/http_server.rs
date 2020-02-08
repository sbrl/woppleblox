use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
pub async fn start(port: i16) -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", port);
    
    info!("Starting listener on http://{}", address);
    
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
