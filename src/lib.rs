#![allow(dead_code)]

extern crate image;

pub mod config;
pub mod parser;

use image::{ImageBuffer, Rgb};

use std::error::Error;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use config::Config;
//use parser::Parser;

/// Given a Configuration, attempts to generate a ray traced image
///
/// # arguements
///
/// * `config` - a configuration for the ray tracer
pub fn run(config: Config) ->Result<(), Box<Error>> {

    // read the input filename and parse for the given scene
    let mut f = File::open(config.ray_filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // TODO: parsing and scene setup

    let (width, height) = config.output_dimensions;
    let image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // TODO: divy up parts of the image for ray tracing

    // TODO: more safe error handling
    image_buf.save(config.output_filename).unwrap();

    Ok(())
}