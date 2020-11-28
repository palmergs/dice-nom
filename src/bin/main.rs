extern crate clap;
use clap::{App, Arg};

use dice_nom::parsers::generator_parser;
use dice_nom::generators::Generator;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("roll")
        .version(VERSION)
        .about("Generates random dice rolls")
        .author("Galen P.")
        .arg(Arg::with_name("INPUT")
            .help("A dice roll expression is required.")
            .required(true)
            .index(1))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    match generator_parser(input) {
        Ok((_, gen)) => println!("{}: {}", gen, gen.generate()),
        Err(_) => println!("could not parse `{}`", input)
    }
}