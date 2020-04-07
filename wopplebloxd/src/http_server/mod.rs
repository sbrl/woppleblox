use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
// use actix_service::Service;
// use futures::future::FutureExt;

mod handlers;
mod routes;

use crate::state::GlobalState;
use crate::settings::{ Settings };
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
    
    
    pub async fn start(&self, port: i16) -> Result<(), std::io::Error> {
        let address = format!("127.0.0.1:{}", port);
        
        handlers::print_embedded_files();
        
        let data = web::Data::new(GlobalState::new(self.settings.clone()));
        
        info!("Starting listener on http://{}", address);
        
        HttpServer::new(move || {
            /*
             * TODO: Plan out the routes here.
             * We may be able to snaffle some of this from the Node.js version.
             */
            App::new()
                .app_data(data.clone())
                .wrap(Logger::default())
                .route("/static/{filepath:.*}", web::get().to(handlers::handle_static))
                .route("/firstrun", web::get().to(routes::RouteFirstRun::main))
                .route("/", web::get().to(index))
        })
            .keep_alive(120) // TODO: Read this from a config file here
            .bind(address)? //.expect("Error: Failed to bind to address (is another processs using it?)")
            .run()
            .await
    }
}

async fn index(state : web::Data<GlobalState>) -> impl Responder {
    let body_text = (templates::TemplateMain {
        title: "Test page",
        content: &format!("Hello, world from {}!", state.sitename)
    }).call().unwrap();
    
    HttpResponse::Ok().body(body_text)
}
