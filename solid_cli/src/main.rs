mod initialize;
mod server;

use clap::{App, Arg};

use initialize::initialize_server_configuration;
use server::start;

fn main() {
    let cli = App::new("solid-rs")
        .version("0.1.0")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(generate_init_subcommand())
        .subcommand(generate_start_subcommand())
        .get_matches();
    
    match cli.subcommand() {
        ("init", optional_matches) => initialize_server_configuration(optional_matches),
        ("start", optional_matches) => start(optional_matches), 
        _   => { }
    }
}

fn generate_init_subcommand() -> App<'static, 'static> {
    App::new("init")
        .about("Setup routine to run a solid-rs server")   
        .arg(Arg::with_name("configuration")
            .short("c")
            .long("configuration")
            .value_name("PATH")
            .help("Path to the configuration file")
            .takes_value(true))
}

fn generate_start_subcommand() -> App<'static, 'static> {
    App::new("start")
        .about("Start a solid-rs server instance")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Port the server will listen on")
            .takes_value(true))
}