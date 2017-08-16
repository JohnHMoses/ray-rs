//! This module provides the structs and functions to parse `.ray` files
//! into a tree-like hierarchy, that can then be easily converted into
//! a generalized Scene description, so that ray_rs::ray_tracer can render
//! the given scene.

use cgmath::{Matrix, Matrix3, Matrix4, Rad, Vector3, Vector4};

use std::iter::Peekable;
use std::result;
use std::slice::Iter;

use super::*;
use super::error::TokenizationError;
use super::ray_tokenizer::{RayTokenizer, Readable, Token};

use super::super::scene::TransformNode;

type Tokenizer<'a> = Peekable<Iter<'a, Token<'a>>>;
type Result<T> = result::Result<T, TokenizationError>;

pub struct RaySceneBuilder {
    lights: Vec<LightBuilder>,
    objects: Vec<TransformableElementBuilder>,
    root_transform: TransformNode,
    camera: Option<CameraBuilder>,
    ambient: Option<Vector3<f64>>,
    // TODO?: material? Parser.cpp has some weird code for
    //        top level material parsering
}

impl RaySceneBuilder {
    pub fn new(tokens: Vec<Token>) -> Result<RaySceneBuilder> {
        let mut peekable_tokens = tokens.iter().peekable();

        RaySceneBuilder {
            lights: Vec::new(),
            objects: Vec::new(),
            root_transform: TransformNode::root(),
            camera: None,
            ambient: None,
        }.parse_scene(&mut peekable_tokens)
    }

    fn parse_scene(mut self, tokenizer: &mut Tokenizer) -> Result<RaySceneBuilder> {
        tokenizer.read( Token::SbtRaytracer )?;

        if let Token::Scalar(version) = tokenizer.read( Token::Scalar(0f64) )? {
            if version > 1.1 {
                return Err(TokenizationError::new("Unsupported SbtRaytracer version"));
            }
        }

        // TODO: this should loop until EOF, then return
        let token_option = tokenizer.peek().map(|t| *t);
        match token_option {
            Some(token) => match *token {
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
                    self.objects.push( TransformableElementBuilder::new(tokenizer, &self.root_transform)? )
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
}

enum TransformableElementType {
    Geometry(Box<GeometryBuilder>),
    Group(GroupBuilder),
}

struct TransformableElementBuilder {
    element: Option<TransformableElementType>,
}

impl TransformableElementBuilder {
    pub fn new(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<TransformableElementBuilder> {
        TransformableElementBuilder {
            element: None,
        }.parse_transformable_element(tokenizer, transform_node)
    }

    fn parse_transformable_element(mut self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<TransformableElementBuilder> {
        let token_option = tokenizer.peek().map(|t| *t);
        match token_option {
            Some(token) => match *token {
                Token::Sphere |
                Token::Box |
                Token::Square |
                Token::Cylinder |
                Token::Cone |
                Token::Trimesh |
                Token::Rotate |
                Token::Scale |
                Token::Transform => {
                    self.element = Some(
                        TransformableElementType::Geometry(Box::new(GeometryBuilder::new(tokenizer, transform_node)?))
                    );
                },
                Token::LBrace => {
                    self.element = Some(
                        TransformableElementType::Group(GroupBuilder::new(tokenizer, transform_node)?)
                    );
                },
                Token::Material => unimplemented!(),
                _ => unimplemented!(), // Syntax error
            },
            None => unimplemented!(), // Syntax error
        }

        Ok(self)
    }
}

struct LightBuilder;
struct CameraBuilder;
struct GeometryBuilder {
   element: Option<GeometryBuilderType>,
}

// TODO: these should hold materials when materials are added
enum GeometryBuilderType {
    ConcreteGeometryType(TransformNode, GeometryType),
    TransformableElement(TransformNode, TransformableElementBuilder),
}

enum GeometryType {
    Sphere,
    Box,
    Square,
    Cylinder,
    Cone { capped: bool, height: f64, bottom_radius: f64, top_radius: f64 },
    Trimesh,
}

// NOTE: Geometry builder doesn't have the option of being empty in a well-formed
//       .ray file. Consider making it handle a generic type T: GeometryBuilderSubtype
impl GeometryBuilder {
    pub fn new(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GeometryBuilder> {
        GeometryBuilder {
            element: None,
        }.parse_geometry(tokenizer, transform_node)
    }

    fn parse_geometry(mut self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GeometryBuilder> {
        let token_option = tokenizer.peek().map(|t| *t);
        match token_option {
            Some(token) => match *token {
                Token::Sphere |
                Token::Box |
                Token::Square |
                Token::Cylinder => self.element = Some(
                    GeometryBuilderType::ConcreteGeometryType(
                        transform_node.clone(),
                        GeometryBuilder::parse_unit_object(tokenizer, token)?)
                    ),
                Token::Cone => self.element = Some(
                    GeometryBuilderType::ConcreteGeometryType(
                        transform_node.clone(),
                        GeometryBuilder::parse_cone(tokenizer)?)
                    ),
                Token::Trimesh => unimplemented!(),
                Token::Translate => self.element = Some(GeometryBuilder::parse_translate(tokenizer, transform_node)?),
                Token::Rotate => self.element = Some(GeometryBuilder::parse_rotate(tokenizer, transform_node)?),
                Token::Scale => self.element = Some(GeometryBuilder::parse_scale(tokenizer, transform_node)?),
                Token::Transform => self.element = Some(GeometryBuilder::parse_transform(tokenizer, transform_node)?),
                _ => unimplemented!(), // syntax error
            },
            None => unimplemented!(), // logic error in parsing
        }

        Ok(self)
    }

    // Sphere, Box, Square, Cylinder
    fn parse_unit_object(tokenizer: &mut Tokenizer, object_type: &Token) ->  Result<GeometryType> {
        // discard the next token, we already know what object we're parsing from the above
        tokenizer.next();
        tokenizer.read( Token::LBrace )?;

        loop {
            let token_option = tokenizer.peek().map(|t| *t);
            match token_option {
                Some(token) => match *token {
                    Token::Material => unimplemented!(),
                    Token::Name => unimplemented!(),
                    Token::RBrace => {
                        tokenizer.read( Token::RBrace )?;
                        // TODO: add material to return_value
                        return match *object_type {
                            Token::Sphere => Ok(GeometryType::Sphere),
                            Token::Box => Ok(GeometryType::Box),
                            Token::Square => Ok(GeometryType::Square),
                            Token::Cylinder => Ok(GeometryType::Cylinder),
                            _ => Err(TokenizationError::new("unexpected token")),
                        }

                    },
                    _ => unimplemented!(), // syntax error, unexpected token
                },
                None => unimplemented!(), // synxtax error, EOF
            }
        }
    }

    fn parse_cone(tokenizer: &mut Tokenizer) ->  Result<GeometryType> {

        let mut capped = true;
        let mut height = 1.0f64;
        let mut bottom_radius = 1.0f64;
        let mut top_radius = 0.0f64;

        tokenizer.read( Token::Cone )?;
        tokenizer.read( Token::RBrace )?;

        loop {
            let token_option = tokenizer.peek().map(|t| *t);
            match token_option {
                Some(token) => match *token {
                    Token::Material => unimplemented!(),
                    Token::Name => unimplemented!(),
                    Token::Capped => capped = parse_boolean_expression(tokenizer)?,
                    Token::Height => height = parse_scalar_expression(tokenizer)?,
                    Token::BottomRadius => bottom_radius = parse_scalar_expression(tokenizer)?,
                    Token::TopRadius => top_radius = parse_scalar_expression(tokenizer)?,

                    Token::RBrace => {
                        tokenizer.read( Token::RBrace )?;
                        // TODO: add material to return_value
                        return Ok(GeometryType::Cone {
                            capped,
                            height,
                            bottom_radius,
                            top_radius,
                        });


                    },
                    _ => unimplemented!(), // syntax error, unexpected token
                },
                None => unimplemented!(), // synxtax error, EOF
            }
        }

    }

    fn parse_translate(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GeometryBuilderType> {
        tokenizer.read( Token::Translate )?;
        tokenizer.read( Token::LParen )?;
        let x = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let y = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let z = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;

        let transform = transform_node.create_child(Matrix4::from_translation(Vector3::new(x, y, z)));
        let subelement = TransformableElementBuilder::new(tokenizer, &transform)?;

        tokenizer.read( Token::RParen )?;
        tokenizer.conditional_read( Token::Semicolon );

        Ok(GeometryBuilderType::TransformableElement(transform, subelement))
    }

    fn parse_rotate(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GeometryBuilderType> {
        tokenizer.read( Token::Rotate )?;
        tokenizer.read( Token::LParen )?;
        let x = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let y = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let z = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let angle = parse_scalar(tokenizer)?;
        tokenizer.read( Token::Comma )?;

        // TODO: double check if glm uses radians or degrees
        let rotation = Matrix3::from_axis_angle(Vector3::new(x, y, z), Rad(angle));
        let transform = transform_node.create_child(Matrix4::from(rotation));
        let subelement = TransformableElementBuilder::new(tokenizer, &transform)?;

        tokenizer.read( Token::RParen )?;
        tokenizer.conditional_read( Token::Semicolon );

        Ok(GeometryBuilderType::TransformableElement(transform, subelement))
    }

    fn parse_scale(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GeometryBuilderType> {
        tokenizer.read( Token::Scale )?;
        tokenizer.read( Token::LParen )?;
        let x = parse_scalar(tokenizer)?;
        let y;
        let z;

        if let Some( &Token::Scalar(_) ) = tokenizer.peek().map(|t| *t) {
            y = parse_scalar(tokenizer)?;
            tokenizer.read( Token::Comma )?;
            z = parse_scalar(tokenizer)?;
            tokenizer.read( Token::Comma )?;
        } else {
            y = x;
            z = x;
        }

        let transform = transform_node.create_child(Matrix4::from_nonuniform_scale(x, y, z));
        let subelement = TransformableElementBuilder::new(tokenizer, &transform)?;

        tokenizer.read( Token::RParen )?;
        tokenizer.conditional_read( Token::Semicolon );

        Ok(GeometryBuilderType::TransformableElement(transform, subelement))
    }

    fn parse_transform(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GeometryBuilderType> {
        tokenizer.read( Token::Transform )?;
        tokenizer.read( Token::LParen )?;
        let row1 = parse_vector4(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let row2 = parse_vector4(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let row3 = parse_vector4(tokenizer)?;
        tokenizer.read( Token::Comma )?;
        let row4 = parse_vector4(tokenizer)?;
        tokenizer.read( Token::Comma )?;

        // TODO: check that these are being put in the right order
        let transform = transform_node.create_child(Matrix4::from_cols(row1, row2, row3, row4).transpose());
        let subelement = TransformableElementBuilder::new(tokenizer, &transform)?;

        tokenizer.read( Token::RParen )?;
        tokenizer.conditional_read( Token::Semicolon );

        Ok(GeometryBuilderType::TransformableElement(transform, subelement))
    }

}

struct GroupBuilder {
    elements: Vec<TransformableElementBuilder>,
}

impl GroupBuilder {
    pub fn new(tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GroupBuilder> {
        GroupBuilder {
            elements: Vec::new(),
        }.parse_group(tokenizer, transform_node)
    }

    fn parse_group(mut self, tokenizer: &mut Tokenizer, transform_node: &TransformNode) -> Result<GroupBuilder> {
        tokenizer.read( Token::LBrace )?;

        loop {
            let token_option = tokenizer.peek().map(|t| *t);
            match token_option {
                Some(token) => match *token {
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
                        self.elements.push( TransformableElementBuilder::new(tokenizer, transform_node)? )
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

        Ok(self)
    }
}

fn parse_boolean_expression(tokenizer: &mut Tokenizer) -> Result<bool> {
    unimplemented!();
}

fn parse_scalar_expression(tokenizer: &mut Tokenizer) -> Result<f64> {
    unimplemented!();
}

fn parse_vector4_expression(tokenizer: &mut Tokenizer) -> Result<Vector4<f64>> {
    unimplemented!();
}

fn parse_boolean(tokenizer: &mut Tokenizer) -> Result<bool> {
    unimplemented!();
}

fn parse_scalar(tokenizer: &mut Tokenizer) -> Result<f64> {
    unimplemented!();
}

fn parse_vector4(tokenizer: &mut Tokenizer) -> Result<Vector4<f64>> {
    unimplemented!();
}