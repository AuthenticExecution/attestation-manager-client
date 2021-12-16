mod config;
mod error;
mod utils;
mod processor;
mod handlers;
mod structs;

use std::str::FromStr;
use clap::{Arg, ArgMatches, App};
use simple_logger::SimpleLogger;
use log::{LevelFilter, debug};

use crate::config::Config;
use manager_net::Command;

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("AttestationManager client")
          .version("1.0")
          .author("Gianluca Scopelliti <gianlu.1033@gmail.com>")
          .about("Client for the Attestation Manager to send requests/data")
          .arg(Arg::with_name("loglevel")
               .short("l")
               .long("loglevel")
               .value_name("LEVEL")
               .help("Log level: {error,warn,info,debug,trace}")
               .takes_value(true)
               .default_value("info"))
          .arg(Arg::with_name("config")
               .short("c")
               .long("config")
               .value_name("FILE")
               .help("Config file (JSON or YAML) that includes the AttestationManager information")
               .takes_value(true)
               .required(true))
          .arg(Arg::with_name("request")
                .short("r")
                .long("request")
                .value_name("COMMAND")
                .help("Select the request type to send to the Attestation Manager")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("data")
                 .short("d")
                 .long("data")
                 .value_name("FILE")
                 .help("JSON/YAML file containing the data to be sent to the Attestation Manager")
                 .takes_value(true)
                 .default_value("dummy"))
          .get_matches()
}

fn main() {
    let matches = parse_args();

    // set log level
    let level = matches.value_of("loglevel").unwrap();
    let level = LevelFilter::from_str(&level).unwrap();
    SimpleLogger::new().with_level(level).init().unwrap();

    // parse config file
    let config = matches.value_of("config").unwrap();
    let config : Config = utils::read_from_file(config).unwrap();

    debug!("{:?}", config);

    // parse request
    let request = matches.value_of("request").unwrap();
    let request = Command::from_str(&request.to_lowercase()).expect("Request not valid");

    // conditional execution according to request
    let data = matches.value_of("data").unwrap();
    processor::execute(config, request, data).unwrap();

    // complete
    debug!("All done!");
}
