use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
// use actix_service::Service;
// use futures::future::FutureExt;

mod global_state;

use global_state::GlobalState;

#[actix_rt::main]
pub async fn start(port: i16) -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", port);
    
    info!("Starting listener on http://{}", address);
    
    HttpServer::new(|| {
        App::new()
            .data(GlobalState::default())
            .wrap(Logger::default())
            .service(index)
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index(state : web::Data<GlobalState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, world from {}!", state.sitename))
}
