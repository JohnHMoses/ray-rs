//! This module provides the structs and functions to parse `.ray` files
//! into a tree-like hierarchy, that can then be easily converted into
//! a generalized Scene description, so that ray_rs::ray_tracer can render
//! the given scene.

use cgmath::{Vector3};

use super::*;
use super::tokenizer::Tokenizer;
use super::tokenizer::token::Token;

pub struct SceneBuilder {
	lights: Vec<LightBuilder>,
	objects: Vec<TransformableObjectBuilder>,
	camera: Option<CameraBuilder>,
	ambient: Option<Vector3<f64>>,
	// TODO?: material? Parser.cpp has some weird code for
	//		  top level material parsering
}

impl SceneBuilder {
	pub fn new() -> SceneBuilder {
		SceneBuilder {
			lights: Vec::new(),
			objects: Vec::new(),
			camera: None,
			ambient: None,
		}
	}

	pub fn parse_scene(&mut self, tokenizer: &mut Tokenizer) {
		// TODO
		//if let Some(Token::SbtRaytracer) = tokenizer.next() {
		//	if let Some()
		//}
		unimplemented!();
	}

	pub fn create_scene(&self) -> Scene {
		// TODO
		Scene::new()
	}

	// TODO: setters
}

struct LightBuilder;
struct TransformableObjectBuilder;
struct CameraBuilder;