mod tokenizer;
mod ray_scene_builder;

use self::tokenizer::Tokenizer;
use self::ray_scene_builder::SceneBuilder;

use super::scene::Scene;

/// Trait a scene-file parser must implement to return a generalized
/// Scene that our ray tracer understands how to render
pub trait Parser {
   fn parse_scene(&mut self) -> Scene;
}

/// Parser for `.ray` files
pub struct RayParser<'a> {
	tokenizer: Tokenizer<'a>,
}

impl<'a> RayParser<'a> {
	/// Given a string slice representing a `.ray` file, creates
	/// a parser that can return a generalized Scene
	pub fn new(input: &str) -> RayParser {
		let tokenizer = Tokenizer::new(input);

		RayParser { tokenizer }
	}
}

impl<'a> Parser for RayParser<'a> {
	fn parse_scene(&mut self) -> Scene {
		let mut scene_builder = SceneBuilder::new();

		// build internal scene representation
		scene_builder.parse_scene(&mut self.tokenizer);

		// render internal representation into generalized Scene
		scene_builder.create_scene()
	}
}