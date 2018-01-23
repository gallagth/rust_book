extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::{ArgMatches};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &ArgMatches) -> Result<Config, &'static str> {
        let query = String::from(args.value_of("query").unwrap());
        let filename = String::from(args.value_of("filename").unwrap().clone());
        let case_sensitive = !args.is_present("case_insensitive");
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    if config.case_sensitive {
        for line in search_case_sensitive(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "dUCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Ductape.";
        assert_eq!(vec!["safe, fast, productive.", "Ductape."], search_case_insensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:"], search_case_sensitive(query, contents));
    }
}
