#[cfg(test)]
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
fn camera_parse_test() {
    use super::*;
    let tokenizer = Tokenizer::new(CAMERA_SAMPLE.chars().peekable());
    let tokens: Vec<token::Token> = tokenizer.collect();

    assert!(tokens.len() > 0)
}