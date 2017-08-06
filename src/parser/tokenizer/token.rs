#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Unknown,                    // Placeholder
    Eofsym,                     // End Of File
    SbtRaytracer,

    Ident(& 'a str),                      // Ident (gets Enclosed In Quotes In Trace Files)

    Symtrue,
    Symfalse,

    LParen, RParen,             // Punctuation
    LBrace, RBrace,
    LBracket, RBracket,
    Comma,
    Minus,
    Equals,
    Semicolon,
    StrLit(& 'a str),
    Scalar(f64),                     // Scalar Values

    Camera,                     // Camera Primitive
    PointLight,                // Lights
    DirectionalLight,
    AmbientLight,

    ConstantAttenuationCoeff, // Terms Affecting The Intensity Dropoff
    LinearAttenuationCoeff,   // Of Point Lights (see The Pointlight 
    QuadraticAttenuationCoeff,// Class)

    Sphere,                     // Primitives
    Box,
    Square,
    Cylinder,
    Cone,
    Trimesh,

    Position, Viewdir,          // Keywords Affecting Primitives
    Updir, Aspectratio,
    Fov,
    Color,
    Direction,
    Capped,
    Height,
    BottomRadius,
    TopRadius,
    Quaternion,                 // ???

    Polypoints, Normals,        // Keywords Affecting Polygons
    Materials, Faces,
    Gennormals,

    Translate, Scale,           // Transforms
    Rotate, Transform,

    Material,                   // Material Settings
    Emissive, Ambient, 
    Specular, Reflective,
    Diffuse, Transmissive,
    Shininess, Index,
    Name,
    Map,
}