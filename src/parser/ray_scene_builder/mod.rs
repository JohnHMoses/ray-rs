//! This module provides the structs and functions to parse `.ray` files
//! into a tree-like hierarchy, that can then be easily converted into
//! a generalized Scene description, so that ray_rs::ray_tracer can render
//! the given scene.

use cgmath::{Vector3};

use super::*;
use super::tokenizer::Tokenizer;
use super::tokenizer::token::Token;

use super::super::scene::TransformNode;

pub struct SceneBuilder {
	lights: Vec<LightBuilder>,
	objects: Vec<TransformableElementBuilder>,
    root_transform: TransformNode,
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
            root_transform: TransformNode::root(),
			camera: None,
			ambient: None,
		}
	}

	pub fn parse_scene(&mut self, tokenizer: &mut Tokenizer) {
		// TODO version parsing at top
		//if let Some(Token::SbtRaytracer) = tokenizer.next() {
		//	if let Some()
		//}
        let token_option = tokenizer.peek();
        match token_option {
            Some(token) => match token {
                Token::Sphere |
                Token::Box |
                Token::Square |
                Token::Cylinder |
                Token::Cone |
                Token::Trimesh |
                Token::Translate |
                Token::Rotate |
                Token::Scale |
                Token::Transform |
                Token::LBrace => {
                    self.objects.push( TransformableElementBuilder::new()
                        .parse_transformable_element(tokenizer, &self.root_transform) );
                },
                Token::PointLight => unimplemented!(),
                Token::DirectionalLight => unimplemented!(),
                Token::AmbientLight => unimplemented!(),
                Token::Camera => unimplemented!(),
                Token::Material => unimplemented!(),
                _ => unimplemented!(), // Synxtax error, unexpected token
            },
            None => unimplemented!(), // EOF
        }
	}

	pub fn create_scene(&self) -> Scene {
		// TODO
        unimplemented!();
	}
	// TODO: setters
}

enum TransformableElementType {
    Geometry(GeometryBuilder),
    Group(GroupBuilder),
}

struct TransformableElementBuilder {
    element: Option<TransformableElementType>,
}

impl TransformableElementBuilder {
    pub fn new() -> TransformableElementBuilder {
        TransformableElementBuilder { element: None }
    }

    pub fn parse_transformable_element(&mut self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> TransformableElementBuilder {
        let token_option = tokenizer.peek();
        match token_option {
            Some(token) => match token {
                Token::Sphere |
                Token::Box |
                Token::Square |
                Token::Cylinder |
                Token::Cone |
                Token::Trimesh |
                Token::Rotate |
                Token::Scale |
                Token::Transform => {
                    self.element = TransformableElementType::Geometry(GeometryBuilder::new(tokenizer, transform_node));
                },
                Token::LBrace => {
                    self.element = TransformableElementType::Group(GroupBuilder::new(tokenizer, transform_node));
                },
                Token::Material => unimplemented!(),
                _ => unimplemented!(), // Syntax error
            },
            None => unimplemented!(), // Syntax error
        }

    }
}

struct LightBuilder;
struct CameraBuilder;
struct GeometryBuilder;
struct GroupBuilder;


