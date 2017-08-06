#![allow(dead_code)]

pub mod config;
pub mod parser;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use config::Config;
//use parser::Parser;

/// Given a Configuration, attempts to generate a ray traced image
///
/// # arguements
///
/// * `config` - a configuration for the ray tracer
pub fn run(config: Config) ->Result<(), Box<Error>> {

    let mut f = File::open(config.ray_filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(())
}