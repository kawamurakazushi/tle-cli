extern crate colored_json;

use colored_json::prelude::*;
use std::{env, process};
use tle_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("There should be only one argument.");
        process::exit(1);
    }

    let query = &args[1];

    let tle = tle_parser::parse(query).unwrap_or_else(|err| {
        eprintln!("Problem parsing tle: {}", err);
        process::exit(1);
    });

    let colored_tle_json = tle.to_json().to_colored_json_auto().unwrap_or_else(|err| {
        eprintln!("Problem parsing tle: {}", err);
        process::exit(1);
    });

    println!("{}", colored_tle_json);
}
