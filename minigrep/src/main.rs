extern crate minigrep;
extern crate clap;

use std::process;
use clap::{Arg, App};

use minigrep::Config;

fn main() {
    let matches = App::new("minigrep")
                    .version("0.0.1")
                    .author("Thomas Gallagher <gallagth@gmail.com>")
                    .about("A shitty grep")
                    .arg(Arg::with_name("case_insensitive")
                        .short("i")
                        .long("case-insensitive")
                        .help("Set for case insensitive search"))
                    .arg(Arg::with_name("query")
                        .help("What to grep for")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("filename")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(2))
                    .get_matches();

    let config = Config::new(&matches).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
