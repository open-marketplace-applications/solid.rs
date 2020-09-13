use clap::ArgMatches;

use solid_server::{start_server, configuration::Configuration};

pub fn start(optional_matches: Option<&ArgMatches>) {
    let mut configuration = load_server_configuration();

    if let Some(matches) = optional_matches {
        apply_cli_arguments(&mut configuration, matches);
    }

    match start_server(configuration) {
        Ok(_) => (),
        Err(error) => println!("Error: {}", error)
    }
}

fn load_server_configuration() -> Configuration {
    /*  ToDo ... 
    *   Load configuration from optional configuration file.
    *   If it exists!
    *   Else we take teh default values of Configuration::new()
    */
    Configuration::new()
}

fn apply_cli_arguments(configuration: &mut Configuration, matches: &ArgMatches) {
    // Cli arguments override default & config file arguments
    if let Some(val) = matches.value_of("port") {
        match val.parse::<u16>() {
            Ok(port) => configuration.server.port = port,
            Err(error) => println!("Error: {}", error)
        }
    }
}