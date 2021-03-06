extern crate clap;
extern crate pretty_env_logger; 

extern crate chrono;
extern crate pinto;
extern crate argonautica;

extern crate fluent;
extern crate accept_language;
#[macro_use]
extern crate refinery;
#[macro_use]
extern crate rusqlite;

#[macro_use]
extern crate log;
#[macro_use]
extern crate rust_embed;

#[macro_use]
extern crate yarte;

mod settings;
mod state;
mod db;
mod intl;

mod helpers_data;
mod helpers_actix;
mod helpers_templating;
// mod helpers_misc;

mod http_server;
mod templates;
mod views;

use std::fs;
use std::process;
use std::path::Path;

use clap::{ Arg, App, SubCommand };
use futures::executor::block_on;

use settings::Settings;

#[actix_rt::main]
async fn main() {
    // Initialise the logging system
    pretty_env_logger::init();
    
    // Initialise the application itself    
    let matches = App::new("Woppleblox")
        .version("0.1")
        .author("Starbeamrainbowlabs")
        .about("The wopplebox server - lightweight and open microblogging app")
        .arg(Arg::with_name("config")
            .short("c").long("config")
            .value_name("path/to/config.toml")
            .help("Specifies the config filename location")
            .takes_value(true))
        .arg(Arg::with_name("config-write")
            .long("config-write")
            .help("If the config file does not exist, create it"))
        .subcommand(SubCommand::with_name("server")
            .about("Starts the server")
            .arg(Arg::with_name("port")
                .short("p").long("port")
                .value_name("port_number")
                .help("Specifies the port number to listen on")
                .takes_value(true)
                .default_value("3500")))
        .get_matches();
    
    // If we are supposed to write the config file if it doesn't exist and we can't find it, write it out
    if matches.is_present("config-write") && matches.is_present("config") && !Path::new(matches.value_of("config").unwrap()).exists() {
        match fs::write(
            "config.toml",
            toml::to_string_pretty(&Settings::default()).expect("Error: Failed to serialise default settings O.o (this is a bug, please get in touch)")
        ) {
            Ok(_) => (),
            Err(error) => {
                warn!("Warning: Didn't find a config file, but failed to write a default one to disk.");
                warn!("Details: {}", error);
            }
        }
    }
    // Create the settings object
    let mut settings : Settings = if matches.is_present("config") {
        Settings::from_file(matches.value_of("config").expect("Error: No config filepath specified (try --help)").to_string())
    }
    else { Settings::new() };
    
    // Load in select CLI arguments
    
    
    if matches.is_present("port") {
        settings.http.port = matches.value_of("port").unwrap()
            .parse().expect("Error: Invalid port number");
    }
    
    match matches.subcommand_name() {
        Some("server") => {
            // Take an explicit copy of the port number
            // This is required because we move the value of the settings variable when creating the WopplebloxApp instance - and hence we can't access it here anymore.
            // We _could_ copy the entirety of settings instead(?), but since we can avoid that we will.
            let port = settings.http.port;
            let app = http_server::WopplebloxApp::new(settings);
            
            
            // Start the HTTP server and handle the result
            // Note that we pass in the port number here to satisfy actix_rt (are we even using it?)
            // block_on is from the futures crate and runs a future (basically a Promise) to completion. The .await syntax is weird - not sure what that actually does just yet.
            // More advanced options are also available. More information: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
            // match block_on(app.start(port)) {
            // match app.start(port) {
            match app.start(port).await {
                Ok(_) => {
                    info!("Server exited normally.");
                },
                Err(message) => {
                    error!("Error: The HTTP server fell over! Details:");
                    error!("{}", message);
                }
            }
        }
        None => {
            println!("Error: No subcommand specified (try --help)");
            process::exit(1);
        },
        _ => {
            println!("Error: Unknown subcommand {} specified", matches.subcommand_name().unwrap());
            process::exit(1);
        }
    }
}
