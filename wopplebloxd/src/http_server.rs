use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
// use actix_service::Service;
// use futures::future::FutureExt;

mod global_state;
mod handlers;

use global_state::GlobalState;
use crate::settings::{Settings};
use crate::templates;
use yarte::Template; // Apparently .render() is part of the trait (who'd have guessed?), so we need to use it here

// TODO: This could be in a class? Or a struct with an impl....
// Then we could take in a (read-only) settings object in the constructor

pub struct WopplebloxApp {
    settings : Settings
}

impl WopplebloxApp {
    pub fn new(settings : Settings) -> WopplebloxApp {
        WopplebloxApp {
            settings : settings
        }
    }
    
    #[actix_rt::main]
    pub async fn start(&self, port: i16) -> std::io::Result<()> {
        let address = format!("127.0.0.1:{}", port);
        
        handlers::print_embedded_files();
        
        info!("Starting listener on http://{}", address);
        
        HttpServer::new(|| {
            /*
             * TODO: Plan out the routes here.
             * We may be able to snaffle some of this from the Node.js version.
             */
            App::new()
                .data(GlobalState::new())
                .wrap(Logger::default())
                .route("/static/{filepath:.*}", web::get().to(handlers::handle_static))
                .route("/", web::get().to(index))
        })
        .keep_alive(120) // TODO: Read this from a config file here
        .bind(address)?
        .run()
        .await
    }
    
    
}

async fn index(state : web::Data<GlobalState>) -> impl Responder {
    let body_text = (templates::MainTemplate {
        title: "Test page",
        content: &format!("Hello, world from {}!", state.sitename)
    }).call().unwrap();
    
    HttpResponse::Ok().body(body_text)
}
