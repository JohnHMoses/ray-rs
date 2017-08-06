extern crate ray_rs;

use std::env;
use std::process;

use ray_rs::config;
use ray_rs::parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    if let Err(e) = ray_rs::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}