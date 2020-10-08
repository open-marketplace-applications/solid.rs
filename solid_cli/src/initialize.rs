use clap::ArgMatches;
use std::{fs::{create_dir_all, File}, io::Write};
use std::path::Path;
use anyhow::Result;

use solid_server::configuration::Configuration;

const DEFAULT_CONFIGURATION_PATH: &str = "./solid-rs-configuration.yaml";

pub fn initialize_server_configuration(optional_matches: Option<&ArgMatches>) -> Result<()> {
    let configuration_path: &Path;

    if let Some(matches) = optional_matches {
        configuration_path = Path::new(matches.value_of("configuration")
            .unwrap_or(DEFAULT_CONFIGURATION_PATH));
    } else {
        configuration_path = Path::new(DEFAULT_CONFIGURATION_PATH);
    }

    initialize_configuration_file(configuration_path)
}

fn initialize_configuration_file(configuration_path: &Path) -> Result<()> {
    let path_as_string = path_to_string(configuration_path);
    if configuration_path.exists() {
        println!("Configuration file {} already exists!", path_as_string)
    } else {
        generate_configuration(configuration_path, path_as_string)
    }
}

fn generate_configuration(configuration_path: &Path, path_as_string: &str) -> Result<()> {
    let configuration = run_setup_wizard();
    println!("Saving configuration to {}", path_as_string);
    create_dir_all(configuration_path.parent().unwrap_or(Path::new("./")))?;
    let configuration_as_bytes = &configuration.to_yaml()?.into_bytes();
    let mut file = File::create(configuration_path)?;
    Ok(file.write_all(configuration_as_bytes)?)
}

fn run_setup_wizard() -> Configuration {
    let configuration = Configuration::new();
    println!("Running setup wizard...");
    // To be done !
    configuration
}

fn path_to_string(path: &Path) -> &str {
    match path.to_str() {
        Some(path_as_string) => path_as_string,
        None => DEFAULT_CONFIGURATION_PATH
    }
}
