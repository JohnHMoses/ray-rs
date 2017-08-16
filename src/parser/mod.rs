mod ray_tokenizer;
mod ray_scene_builder;
pub mod error;

use std::error::Error;

use self::ray_tokenizer::RayTokenizer;
use self::ray_scene_builder::RaySceneBuilder;

use super::scene::Scene;

/// Trait a scene-file parser must implement to return a generalized
/// Scene that our ray tracer understands how to render
pub trait Parser<E> {
   fn parse_scene(input: &str) -> Result<Scene, E>
       where E: Error;
}

/// Parser for `.ray` files
pub struct RayParser;

impl Parser<error::TokenizationError> for RayParser {
	fn parse_scene(input: &str) -> Result<Scene, error::TokenizationError> {
        // construct a tokenizer with out input
		let tokenizer = RayTokenizer::new(input);

        // tokenize input and check for token errors
        let tokens = tokenizer.collect::<Result<Vec<_>, error::TokenizationError>>()?;

		// build internal scene representation and check for syntax errors
		let scene_builder = RaySceneBuilder::new(tokens)?;

		// render internal representation into generalized Scene and return it
		Ok(scene_builder.create_scene())
	}
}
