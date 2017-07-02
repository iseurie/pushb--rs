extern crate clap;
extern crate pushb;

use std::io::{self, Read};
use clap::{App, Arg};
use pushb::{Target, Note, Push};

fn main() {
    let arg_matches = App::new("")
        .arg(Arg::with_name("key")
            .short("k")
            .value_name("API_KEY")
            .help("sets the API key")
            .required(true)
        ).arg(Arg::with_name("email")
            .short("e")
            .value_name("EMAIL")
            .help("sets the target email; defaults to the owner of the API key")
        ).get_matches();
        // TODO: Add file/link flags
    let key = arg_matches.value_of("key").unwrap();
    let email = arg_matches.value_of("email");
    let target = match email {
        None => Target::None,
        Some(v) => Target::Email(v.to_string()),
    };

    let mut ibuf = String::new();
    io::stdin().read_to_string(&mut ibuf).unwrap();
    let note = Note { body: ibuf, target: target, ..Default::default() };
    if let Err(err) = note.push(&key) {
        println!("error: {}", err);
    }
}
