mod brainfuck;

extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::path::Path;
use std::io::Read;
use brainfuck::{interprete, allowed};

fn main() {
    let matches = App::new("bfc")
        .version("0.0.1")
        .author("Pavlo Tokariev https://github.com/PaulRaUnite")
        .about("Brainfuck interpreter, 32768 bytes for your apps.")
        .arg(Arg::with_name("INPUT")
            .help("Sets income file for interpreting.")
            .required(true)
            .index(1)
        )
        .get_matches();
    let filepath = match matches.value_of("INPUT") {
        None => {
            eprintln!("here is no INPUT argument");
            return;
        }
        Some(path) => path,
    };

    let path = Path::new(filepath);
    let mut file = match File::open(path) {
        Err(e) => {
            eprintln!("can't open file {}: {}", filepath, e);
            return;
        }
        Ok(file) => file,
    };
    let mut program = Vec::<u8>::new();
    if let Err(e) = file.read_to_end(&mut program) {
        eprintln!("can't read from {}: {}", filepath, e);
        return;
    }
    if let None = allowed(&program) {
        eprintln!("the program have symbols that Brainfuck doesn't contain");
        return;
    }

    if let None = interprete(&mut program) {
        eprintln!("something went wrong")
    };
}