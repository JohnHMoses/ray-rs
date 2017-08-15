//! This module provides the structs and functions to parse `.ray` files
//! into a tree-like hierarchy, that can then be easily converted into
//! a generalized Scene description, so that ray_rs::ray_tracer can render
//! the given scene.

use cgmath::{Vector3};

use std::iter::Peekable;
use std::slice::Iter;

use super::*;
use super::error::TokenizationError;
use super::ray_tokenizer::RayTokenizer;
use super::ray_tokenizer::Token;

use super::super::scene::TransformNode;

type Tokenizer<'a> = Peekable<Iter<'a, Token<'a>>>;

pub struct RaySceneBuilder {
	lights: Vec<LightBuilder>,
	objects: Vec<TransformableElementBuilder>,
    root_transform: TransformNode,
	camera: Option<CameraBuilder>,
	ambient: Option<Vector3<f64>>,
	// TODO?: material? Parser.cpp has some weird code for
	//		  top level material parsering
}

impl RaySceneBuilder {
	pub fn new(tokenizer: RayTokenizer) -> Result<RaySceneBuilder, TokenizationError> {
        let tokens = tokenizer.collect::<Result<Vec<_>, TokenizationError>>()?;
        let mut peekable_tokens = tokens.iter().peekable();

		RaySceneBuilder {
			lights: Vec::new(),
			objects: Vec::new(),
            root_transform: TransformNode::root(),
			camera: None,
			ambient: None,
		}.parse_scene(&mut peekable_tokens)
	}

	fn parse_scene(self, tokenizer: &mut Tokenizer) -> Result<RaySceneBuilder, TokenizationError> {
		// TODO version parsing at top
		//if let Some(Token::SbtRaytracer) = tokenizer.next() {
		//	if let Some()
		//}
        // TODO: this should loop until EOF, then return
        let token_option = tokenizer.peek();
        match token_option {
            Some(&&token) => match token {
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
                    self.objects.push( TransformableElementBuilder::new(tokenizer, &self.root_transform) )
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

        Ok(self)
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
    pub fn new(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> TransformableElementBuilder {
        TransformableElementBuilder {
            element: None,
        }.parse_transformable_element(tokenizer, transform_node)
    }

    fn parse_transformable_element(self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> TransformableElementBuilder {
        let token_option = tokenizer.peek();
        match token_option {
            Some(&&token) => match token {
                Token::Sphere |
                Token::Box |
                Token::Square |
                Token::Cylinder |
                Token::Cone |
                Token::Trimesh |
                Token::Rotate |
                Token::Scale |
                Token::Transform => {
                    // TODO: check if ; is necessary
                    self.element = Some(TransformableElementType::Geometry(GeometryBuilder::new(tokenizer, transform_node)))
                },
                Token::LBrace => {
                    self.element = Some(TransformableElementType::Group(GroupBuilder::new(tokenizer, transform_node)))
                },
                Token::Material => unimplemented!(),
                _ => unimplemented!(), // Syntax error
            },
            None => unimplemented!(), // Syntax error
        }

        self
    }
}

struct LightBuilder;
struct CameraBuilder;
struct GeometryBuilder {
   element: Option<GeometryBuilderSubtype>,
}

// NOTE: Geometry builder doesn't have the option of being empty in a well-formed
//       .ray file. Consider making it handle a generic type T: GeometryBuilderSubtype
//       and have a single new_and_parse_geometry() method.
impl GeometryBuilder {
    pub fn new(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> GeometryBuilder {
        GeometryBuilder {
            element: None,
        }.parse_geometry(tokenizer, transform_node)
    }

    fn parse_geometry(self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> GeometryBuilder {
        let token_option = tokenizer.peek();
        match token_option {
            Some(&&token) => match token {
                Token::Sphere => unimplemented!(),
                Token::Box => unimplemented!(),
                Token::Square => unimplemented!(),
                Token::Cylinder => unimplemented!(),
                Token::Cone => unimplemented!(),
                Token::Trimesh => unimplemented!(),
                Token::Translate => unimplemented!(),
                Token::Rotate => unimplemented!(),
                Token::Scale => unimplemented!(),
                Token::Transform => unimplemented!(),
                _ => unimplemented!(), // syntax error
            },
            None => unimplemented!(), // logic error in parsing
        }

        self
    }

    fn parse_unit_object(tokenizer: &mut Tokenizer, transform_node: &TransformNode) ->  GeometryBuilderSubtype {
        tokenizer.read(Token::Sphere)?;
        tokenizer.read(Token::LBrace)?;

        loop {
            let token_option = tokenizer.peek();
            match token_option {
                Some(&&token) => match token {
                    Token::Material => unimplemented!(),
                    Token::Name => unimplemented!(),
                    Token::RBrace => unimplemented!(),
                    _ => unimplemented!(), // syntax error, unexpected token
                },
                None => unimplemented!(), // synxtax error, EOF
            }
        }
    }
}

struct GroupBuilder {
    elements: Vec<TransformableElementBuilder>,
}

impl GroupBuilder {
    pub fn new(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> GroupBuilder {
        GroupBuilder {
            elements: Vec::new(),
        }.parse_group(tokenizer, transform_node)
    }

    fn parse_group(self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> GroupBuilder {
        tokenizer.read( Token::LBrace )?;

        loop {
            let token_option = tokenizer.peek();
            match token_option {
                Some(&&token) => match token {
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
                        self.elements.push( TransformableElementBuilder::new(tokenizer, transform_node) )
                    },
                    Token::RBrace => {
                        tokenizer.read( Token::RBrace )?;
                        break;
                    },
                    Token::Material => unimplemented!(),
                    _ => unimplemented!(), // syntax error
                },
                None => unimplemented!(), // unexpected EOF
            }
        }

        self
    }
}

// TODO: these should hold materials when materials are added
enum GeometryBuilderSubtype {
    Sphere,
    Box,
    Square,
    Cylinder,
    Cone,
    Trimesh,
    Translate,
    Rotate,
    Scale,
    Transform,
}

struct SphereBuilder;
impl SphereBuilder {

}

