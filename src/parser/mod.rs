mod ray_tokenizer;
mod ray_scene_builder;
pub mod error;

use self::ray_tokenizer::RayTokenizer;
use self::ray_scene_builder::RaySceneBuilder;

use super::scene::Scene;

/// Trait a scene-file parser must implement to return a generalized
/// Scene that our ray tracer understands how to render
pub trait Parser {
   fn parse_scene(&mut self) -> Scene;
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

impl<'a> Parser for RayParser<'a> {
	fn parse_scene(&mut self) -> Scene {
		// build internal scene representation
		let mut scene_builder = RaySceneBuilder::new(&mut self.tokenizer);

		// render internal representation into generalized Scene and return it
		scene_builder.create_scene()
	}
}
