use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
// use actix_service::Service;
// use futures::future::FutureExt;

mod global_state;
mod handlers;

use global_state::GlobalState;
use crate::settings::{SettingsManager};

// TODO: This could be in a class? Or a struct with an impl....
// Then we could take in a (read-only) settings object in the constructor

pub struct WopplebloxApp {
    settings : SettingsManager
}

impl WopplebloxApp {
    pub fn new(settings : SettingsManager) -> WopplebloxApp {
        WopplebloxApp {
            settings : settings
        }
    }
    
    #[actix_rt::main]
    pub async fn start(&self, port: i16) -> std::io::Result<()> {
        let address = format!("127.0.0.1:{}", port);
        
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
    HttpResponse::Ok().body(format!("Hello, world from {}!", state.sitename))
}
