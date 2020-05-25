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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let stdin = io::stdin();
    let stdout = io::stdout();
    let input_file = matches.value_of("INPUT");

    let input: Box<dyn io::BufRead> = match input_file {
        Some(file_name) => {
            let f = File::open(file_name)?;
            Box::new(io::BufReader::new(f))
        }
        None => Box::new(stdin.lock()),
    };

    for result in input.lines() {
        let token = JWT::new(result?)?;
        serde_json::to_writer_pretty(stdout.lock(), &token.header)?;
        serde_json::to_writer_pretty(stdout.lock(), &token.body)?;
        if matches.is_present("signature") {
            eprintln!("{}", &token.signature);
        }
    }

    Ok(())
}
