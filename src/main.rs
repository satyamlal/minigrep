mod run;

use run::*;
use std::{env, process};

use crate::run::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Seaching for the word: '{}'", config.query);
    println!("In file: '{}'", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error, {}", e);
        process::exit(1);
    }
}
