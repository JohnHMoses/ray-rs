pub mod token;
pub mod tests;
use std::iter::Iterator;
use regex::Regex;


use self::token::Token;

pub struct Tokenizer<'a> {
    /// input stream
    input: &'a str,
    /// last read token, if it exists
    un_get_token: Option<token::Token<'a>>,

    position: usize
}

impl<'a> Tokenizer<'a>  {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, position: 0, un_get_token: None }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        // Eliminate whitespace
        self.lex_whitespace();
        if self.position >= self.input.len() {
            return None;
        }
        match self.input.chars().nth(self.position).unwrap() {
            '\'' => self.lex_strlit(),
            ';' => {
                self.position += 1;
                Some(Token::Semicolon)
            },
            ',' => {
                self.position += 1;
                Some(Token::Comma)
            },
            '{' => {
                self.position += 1;
                Some(Token::LBrace)
            },
            '}' => {
                self.position += 1;
                Some(Token::RBrace)
            },
            '(' => {
                self.position += 1;
                Some(Token::LParen)
            },
            ')' => {
                self.position += 1;
                Some(Token::RParen)
            },
            '=' => {
                self.position += 1;
                Some(Token::Equals)
            },
            '-' => {
                self.position += 1;
                Some(Token::Minus)
            }
            x if x.is_alphabetic() => self.lex_bareword(),
            x if x.is_numeric() => self.lex_numlit(),
            x => panic!("Couldn't parse at {}:{}", self.position, x)
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

    fn lex_strlit(&mut self) -> Option<Token<'a>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^'([^']|\\')*'").unwrap();
        }
        if let Some(result) = RE.find(&self.input[self.position..]) {
            let start = self.position + result.start();
            let end = self.position + result.end();
            let token = Some(Token::StrLit(&self.input[start..end]));
            self.position = end;
            return token
        }
        None
    }
    fn lex_numlit(&mut self) -> Option<Token<'a>> {
       lazy_static! {
            static ref RE: Regex = Regex::new(r"[[:digit:]]").unwrap();
        }
        if let Some(result) = RE.find(&self.input[self.position..]) {
            let start = self.position + result.start();
            let end = self.position + result.end();
            let value = self.input[start..end].parse().unwrap();
            let token = Some(Token::Scalar(value));
            self.position = end;
            return token
        }
        None
    }
    fn lex_bareword(&mut self) -> Option<Token<'a>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[[:alpha:]]+[[[:alpha:]]-_]*").unwrap();
        }
        if let Some(result) = RE.find(&self.input[self.position..]) {
            let start = self.position + result.start();
            let end = self.position + result.end();
            let value = &self.input[start..end];
            //println!("{}:{}", start, value);
            let token = match value {
                "camera" => Some(Token::Camera),
                "point_light" => Some(Token::PointLight),
                "directional_light" => Some(Token::DirectionalLight),
                "ambient_light" => Some(Token::AmbientLight),
                "sphere" => Some(Token::Sphere),
                "box" => Some(Token::Box),
                "cylinder" => Some(Token::Cylinder),
                "cone" => Some(Token::Cone),
                "trimesh" => Some(Token::Trimesh),
                "SBT-raytracer" => Some(Token::SbtRaytracer),
                _ => Some(Token::Ident(value))
            };
            self.position = end;
            return token
        }
        return None
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
    fn next(& mut self) -> Option<Self::Item> {
        return self.next_token();
    }

}

