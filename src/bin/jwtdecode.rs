/// jwtdecode is a command line tool for inspecting JWT tokens.
///
///

extern crate clap;
use clap::{App, Arg};

extern crate jwtdecode;
use jwtdecode::jwt::JWT;

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let matches = App::new("jwtdecode")
        .version("0.1.0")
        .about("Decode JWT tokens")
        .arg(
            Arg::with_name("INPUT")
                .help("The input file to use. Defaults to STDIN")
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("signature")
                .long("show-signature")
                .short("s")
                .takes_value(false)
                .use_delimiter(false)
                .help("Print the token signature to STDERR"),
        )
        .get_matches();

    let input_file = matches.value_of("INPUT");

    let results: Vec<String> = match input_file {
        Some(file_name) => {
            let f = File::open(file_name).unwrap();
            let mut lines = vec![];
            for line in io::BufReader::new(f).lines() {
                match line {
                    Ok(line) => lines.push(line),
                    _ => continue,
                }
            }
            lines
        }
        _ => {
            let mut lines = vec![];
            for line in io::stdin().lock().lines() {
                match line {
                    Ok(line) => lines.push(line),
                    _ => continue,
                }
            }
            lines
        }
    };
    for result in results {
        let jwt = JWT::new(result.as_str());
        match jwt {
            Ok(token) => {
                let header = serde_json::to_string_pretty(&token.header).unwrap();
                println!("{}", header);
                let body = serde_json::to_string_pretty(&token.body).unwrap();
                println!("{}", body);
                if matches.is_present("signature") {
                    eprintln!("{}", &token.signature);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
