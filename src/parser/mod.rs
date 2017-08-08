mod tokenizer;

use self::tokenizer::Tokenizer;
use super::scene::Scene;

pub struct Parser<'a> {
	tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
	pub fn new(input: &str) -> Parser {
		let tokenizer = Tokenizer::new(input);

		Parser { tokenizer }
	}

	pub fn parse_scene(&self) -> Scene {
		let scene = Scene::new();
		scene
	}
}