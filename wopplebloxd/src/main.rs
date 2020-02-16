extern crate clap;
extern crate pretty_env_logger; 

#[macro_use]
extern crate log;
#[macro_use]
extern crate rust_embed;

mod http_server;
mod settings;

use std::process;

use clap::{Arg, App, SubCommand};

fn main() {
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
            .takes_value(true)
            .default_value("config.toml"))
        .arg(Arg::with_name("config-write")
            .long("config-write")
            .help("If the config file does not exist, create it"))
        .subcommand(SubCommand::with_name("server")
            .about("Starts the server"))
            .arg(Arg::with_name("port")
                .short("p").long("port")
                .value_name("port_number")
                .help("Specifies the port number to listen on")
                .takes_value(true)
                .default_value("3500"))
        .get_matches();
    
    // Create the settings object
    let mut settings = settings::Settings::new();
    
    // Load in select CLI arguments
    if matches.is_present("config-write") { settings.config_write = true; }
    if matches.is_present("port") {
        settings.http.port = matches.value_of("port").unwrap()
            .parse().expect("Error: Invalid port number");
    }
    
    // Load the config file
    settings.load_settings_file(matches.value_of("config").expect("Error: No config filepath specified (try --help)").to_string());
    
    // TODO: Read the config file here
    
    match matches.subcommand_name() {
        Some("server") => {
            
            
            let app = http_server::WopplebloxApp::new(settings);
            
            // Start the HTTP server and handle the result
            // Note that we pass in the port number here to satisfy actix_rt (are we even using it?)
            match app.start(settings.http.port) {
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
