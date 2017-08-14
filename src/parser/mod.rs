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
   fn parse_scene(&mut self) -> Result<Scene, E>
       where E: Error;
}

/// Parser for `.ray` files
pub struct RayParser<'a> {
	tokenizer: RayTokenizer<'a>,
}

impl<'a> RayParser<'a> {
	/// Given a string slice representing a `.ray` file, creates
	/// a parser that can return a generalized Scene
	pub fn new(input: &str) -> RayParser {
		let tokenizer = RayTokenizer::new(input);

		RayParser { tokenizer }
	}
}

impl<'a> Parser<error::TokenizationError> for RayParser<'a> {
	fn parse_scene(&mut self) -> Result<Scene, error::TokenizationError> {
		// build internal scene representation
		let mut scene_builder = RaySceneBuilder::new(self.tokenizer)?;

		// render internal representation into generalized Scene and return it
		Ok(scene_builder.create_scene())
	}
}
