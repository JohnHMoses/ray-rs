#[cfg(test)]
use super::token::Token::*;
static KEYWORD_SAMPLE: &'static str = "camera point_light";
static WHITESPACE_SAMPLE: &'static str = " \t\t\n\r";
static NUMBERS_SAMPLE: &'static str = "-10 500.00001 -0.0 9";
static PUNCTUATION_SAMPLE: &'static str = ",;(){}";

static CAMERA_SAMPLE: &'static str = "camera {
	position = (4,0,0);
	viewdir = (-1,0,0);
	aspectratio = 1;
	updir = (0,1,0);
}";


static RAY_HEADER: &'static str = "SBT-raytracer 1.0";

static POINT_LIGHT_SAMPLE: &'static str = "point_light {
    position = (4, 4, 0);
    color = (.5, .5, .5);
    constant_attenuation_coeff= 0.25;
    linear_attenuation_coeff = 0.003372407;
    quadratic_attenuation_coeff = 0.000045492;
}";

static DIRECTIONAL_LIGHT_SAMPLE: &'static str = " directional_light {
    direction = (0, -1, 0);
    colour = (1.0, 1.0, 1.0);
}";

static BOX_SAMPLE: &'static str = "
box {
    material = {
        //diffuse = (0.7, 0, 1.0);
        specular = (0.9,0.4,0.0);
        shininess = 76.8;
    };
}";

#[test]
fn keyword_tokenize_test() {
    use super::*;
    let tokenizer = Tokenizer::new(KEYWORD_SAMPLE);
    let tokens: Vec<token::Token> = tokenizer.collect();
    let expected = [Camera, PointLight];
    assert!(tokens.eq(&expected));
}
#[test]
fn whitespace_tokenize_test() {
    use super::*;
    let tokenizer = Tokenizer::new(WHITESPACE_SAMPLE);
    let tokens: Vec<token::Token> = tokenizer.collect();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn number_tokenize_test() {
    use super::*;
    let tokenizer = Tokenizer::new(NUMBERS_SAMPLE);
    let tokens: Vec<token::Token> = tokenizer.collect();
    let expected = [Scalar(-10f64), Scalar(500.00001f64), Scalar(0.0f64), Scalar(9f64)];
    assert!(tokens.iter().eq(expected.iter()));
}
#[test]
fn punctuation_tokenize_test() {
    use super::*;
    let tokenizer = Tokenizer::new(PUNCTUATION_SAMPLE);
    let tokens: Vec<token::Token> = tokenizer.collect();
    let expected = [Comma, Semicolon, LParen, RParen, LBrace, RBrace];
    println!("{:?}", tokens);
    println!("{:?}", expected.iter());
    assert!(tokens.iter().eq(expected.iter()));
}

#[test]
fn camera_parse_test() {
    use super::*;
    let tokenizer = Tokenizer::new(CAMERA_SAMPLE);
    let tokens: Vec<token::Token> = tokenizer.collect();
    let expected = [Camera, LBrace,
        Ident("position"), Equals, LParen, Scalar(4f64), Comma, Scalar(0f64), Comma, Scalar(0f64), RParen, Semicolon,
        Ident("viewdir"), Equals, LParen, Minus, Scalar(1f64), Comma, Scalar(0f64), Comma, Scalar(0f64), RParen, Semicolon,
        Ident("aspectratio"), Equals, Scalar(1f64), Semicolon,
        Ident("updir"), Equals, LParen, Scalar(0f64), Comma, Scalar(1f64), Comma, Scalar(0f64), RParen, Semicolon,
        RBrace];
    //println!("{:?}", tokens);
    //println!("{:?}", expected.iter());
    assert!(tokens.iter().eq(expected.iter()))
}