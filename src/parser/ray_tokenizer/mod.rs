pub mod tests;

mod token;

pub use self::token::Token;

use std::iter::{Iterator, Peekable};
use std::slice::Iter;

use regex::Regex;

use super::error::TokenizationError;

pub trait Readable<'a> {
    /// Given an input token, returns that token if the next token matches it.
    /// If it doesn't match, returns an error.
    /// Use when there is only one possible token that should be read next.
    fn read(&mut self, pattern: Token<'a>) -> Result<Token<'a>, TokenizationError>;
    fn conditional_read(&mut self, pattern: Token<'a>) -> bool;
}

pub struct RayTokenizer<'a> {
    /// input stream
    input: &'a str,

    position: usize
}

impl<'a> RayTokenizer<'a>  {

    pub fn new(input: &'a str) -> RayTokenizer<'a> {
        RayTokenizer { input, position: 0}
    }

    fn next_token(&mut self) -> Result<Token<'a>, TokenizationError> {
        // Eliminate whitespace
        self.lex_whitespace();
        if self.position >= self.input.len() {
            return Err(TokenizationError::new("Out"));
        }
        match self.input.chars().nth(self.position).unwrap() {
            '\'' => self.lex_bareword(),
            ';' => {
                self.position += 1;
                Ok(Token::Semicolon)
            },
            ',' => {
                self.position += 1;
                Ok(Token::Comma)
            },
            '{' => {
                self.position += 1;
                Ok(Token::LBrace)
            },
            '}' => {
                self.position += 1;
                Ok(Token::RBrace)
            },
            '(' => {
                self.position += 1;
                Ok(Token::LParen)
            },
            ')' => {
                self.position += 1;
                Ok(Token::RParen)
            },
            '=' => {
                self.position += 1;
                Ok(Token::Equals)
            },
            '-' => {
                self.position += 1;
                self.lex_numlit()
            },
            '.' => {
                Err(TokenizationError::new("blah"))
            },
            x if x.is_alphabetic() => self.lex_bareword(),
            x if x.is_numeric() => self.lex_numlit(),
            x => Err(TokenizationError::new(format!("Couldn't parse at {}:{}", self.position, x)))
        }
    }

    fn lex_whitespace(&mut self) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*").unwrap();
        }

       if let Some(result) = RE.find(&self.input[self.position..]) {
            self.position += result.end();
        }
    }

    fn lex_strlit(&mut self) -> Result<Token<'a>, TokenizationError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^'([^']|\\')*'").unwrap();
        }
        if let Some(result) = RE.find(&self.input[self.position..]) {
            let start = self.position + result.start();
            let end = self.position + result.end();
            let token = Token::StrLit(&self.input[start..end]);
            self.position = end;
            return Ok(token)
        }
        Err(TokenizationError::new("Did not find valid string literal"))
    }
    fn lex_numlit(&mut self) -> Result<Token<'a>, TokenizationError> {
       lazy_static! {
            static ref RE: Regex = Regex::new(r"[[:digit:]]").unwrap();
        }
        if let Some(result) = RE.find(&self.input[self.position..]) {
            let start = self.position + result.start();
            let end = self.position + result.end();
            let value = self.input[start..end].parse().unwrap();
            let token = Token::Scalar(value);
            self.position = end;
            return Ok(token)
        }
        Err(TokenizationError::new("Did not find valid number"))
    }
    fn lex_bareword(&mut self) -> Result<Token<'a>, TokenizationError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[[:alpha:]]+[[[:alpha:]]-_]*").unwrap();
        }
        if let Some(result) = RE.find(&self.input[self.position..]) {
            let start = self.position + result.start();
            let end = self.position + result.end();
            let value = &self.input[start..end];
            //println!("{}:{}", start, value);
            let token = match value {
                "camera" => Token::Camera,
                "point_light" => Token::PointLight,
                "directional_light" => Token::DirectionalLight,
                "ambient_light" => Token::AmbientLight,
                "sphere" => Token::Sphere,
                "box" => Token::Box,
                "cylinder" => Token::Cylinder,
                "cone" => Token::Cone,
                "trimesh" => Token::Trimesh,
                "SBT-raytracer" => Token::SbtRaytracer,
                _ => Token::Ident(value)
            };
            self.position = end;
            return Ok(token)
        }
        Err(TokenizationError::new("Couldn't pares bareword"))
    }
}

impl<'a> Iterator for RayTokenizer<'a> {
    type Item = Result<Token<'a>, TokenizationError>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
        // TODO: Correctly distinguish between the out of tokens state and an error
        if next.is_ok(){
            return Some(next)
        }
        return None
    }
}

impl<'a> Readable<'a> for Peekable<Iter<'a, Token<'a>>> {
    /// Given an input token, returns that token if the iterator's next()
    /// call also return that token, else it returns an error
    fn read(&mut self, pattern: Token<'a>) -> Result<Token<'a>, TokenizationError> {
        if let Some(next_token) = self.next() {
            let copied_token = (*next_token).clone();
            if token_discriminant_equal(pattern, copied_token) {
                return Ok((*next_token).clone());
            } else {
                return Err(TokenizationError::new("unexpected token"));
            }
        } else {
            return Err(TokenizationError::new("out of tokens"));
        }
    }

    fn conditional_read(&mut self, pattern: Token<'a>) -> bool {
        if let Some(token) = self.peek().map(|t| *t) {
            if token_discriminant_equal(pattern, (*token).clone()) {
                self.next();
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

fn token_discriminant_equal(lhs: Token, rhs: Token) -> bool {
    match lhs {
        Token::Ident(_) => {
            if let Token::Ident(_) = rhs {
                return true;
            } else {
                return false;
            }
        },
        Token::StrLit(_) => {
            if let Token::StrLit(_) = rhs {
                return true;
            } else {
                return false;
            }
        },
        Token::Scalar(_) => {
            if let Token::Scalar(_) = rhs {
                return true;
            } else {
                return false;
            }
        },
        lhs => {
            if lhs == rhs {
                return true;
            } else {
                return false;
            }
        }
    }
}