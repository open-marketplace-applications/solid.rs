use clap::{App, Arg, ArgMatches};

fn main() {
    let cli = App::new("solid-rs")
        .version("0.1.0")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("init")
            .about("Setup routine to run a solid-rs server"))   
        .subcommand(App::new("start")
            .about("Start a solid-rs server instance")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Port the server will listen on")
                .takes_value(true)))
        .get_matches();
    
    match cli.subcommand() {
        ("init", _) => init_server(),
        ("start", optional_matches) => start_server(optional_matches), 
        _   => { }
    }
}

fn init_server() {
    println!("Running setup wizard...")
}

fn start_server(optional_matches: Option<&ArgMatches>) {
    let default_port = "80";
    if let Some(matches) = optional_matches {
        let port = matches.value_of("port").unwrap_or(default_port);
        println!("Server start listening on port :{}", port)
    } else {
        println!("Server start listening on default port :{}", default_port)
    }
}