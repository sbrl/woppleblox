extern crate clap;

extern crate pretty_env_logger; 
#[macro_use]
extern crate log;

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
        .subcommand(SubCommand::with_name("server")
            .about("Starts the server"))
            .arg(Arg::with_name("port")
                .short("p").long("port")
                .value_name("port_number")
                .help("Specifies the port number to listen on")
                .takes_value(true)
                .default_value("3500"))
        .get_matches();
    
    
    // TODO: Read the config file here
    
    match matches.subcommand_name() {
        Some("server") => {
            let port_number = matches.value_of("port").unwrap()
                .parse().expect("Error: Invalid port number");
            
            let app = http_server::WopplebloxApp::new();
            
            // Start the HTTP server and handle the result
            match app.start(port_number) {
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
