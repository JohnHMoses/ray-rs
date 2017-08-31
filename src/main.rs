extern crate ray;

use std::env;
use std::process;

use ray::config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ray::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}