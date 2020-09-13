use clap::ArgMatches;
use std::{fs::{create_dir_all, File}, io::Write};
use std::path::Path;

use solid_server::configuration::Configuration;

const DEFAULT_CONFIGURATION_PATH: &str = "./solid-rs-configuration.yaml";

pub fn initialize_server_configuration(optional_matches: Option<&ArgMatches>) {
    let configuration_path: &Path;

    if let Some(matches) = optional_matches {
        configuration_path = Path::new(matches.value_of("configuration")
            .unwrap_or(DEFAULT_CONFIGURATION_PATH));
    } else {
        configuration_path = Path::new(DEFAULT_CONFIGURATION_PATH);
    }

    initialize_configuration_file(configuration_path);
}

fn initialize_configuration_file(configuration_path: &Path) {
    if configuration_path.exists() {
        println!("Configuration file {} already exists!", configuration_path.to_str().unwrap());
    } else {
        let configuration = run_setup_wizard();
        println!("Saving configuration to {}", configuration_path.to_str().unwrap());
        match create_dir_all(configuration_path.parent().unwrap()) {
            Ok(()) => {
                match File::create(configuration_path) {
                    Ok(mut file) => file.write_all(&configuration.to_yaml().unwrap().into_bytes()).unwrap(),
                    Err(error) => println!("Error: {}", error)
                }
            },
            Err(error) =>  println!("Error: {}", error)
        };
    }
}

fn run_setup_wizard() -> Configuration {
    let configuration = Configuration::new();
    println!("Running setup wizard...");
    // To be done !
    configuration
}