use tower_web::ServiceBuilder;

use tower_web::middleware::log::LogMiddleware;
//use tokio::prelude::*;

pub fn start(port: i16) {
    let _ = env_logger::try_init();
    
    let address = format!("127.0.0.1:{}", port).parse().expect("Error: invalid bind address");
    println!("Starting listener on http://{}", address);
    
    ServiceBuilder::new()
        .resource(WoppleBloxApp{})
        .middleware(LogMiddleware::new("woppleblox::WoppleBloxApp"))
        .run(&address)
        .unwrap();
}


#[derive(Clone, Debug)]
pub struct WoppleBloxApp {
    
}

impl_web! {
    impl WoppleBloxApp {
        #[get("/")]
        fn index(&self) -> Result<&'static str, ()> {
            Ok("Hello, world!")
        }
    }
}
